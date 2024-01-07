use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{mpsc, Arc, Mutex};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use futures::task::ArcWake;

struct Delay {
    when: Instant,
}
fn main() {
    let mut mini_tokio = MiniTokio::new();
    mini_tokio.spwan(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };

        let out = future.await;

        assert_eq!(out, "done");
    });

    mini_tokio.run();
}

// struct MiniTokio {
//     tasks: VecDeque<Task>,
//     // Double ended queue that contain Task type
// }
// associated type that pin with future that produce a output as () and implemnet Send trait. bucause task will send between thread
// and have to need thread safety.
// type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

struct MiniTokio {
    scheduled: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,
}

struct Task {
    // The mutex is to make task implemnt sync. only
    // one thread access future at any given time. the
    // mutex is not reqird for correctness. Real tokio
    // does not use a mutex here but real tokio has
    // more lines of code than ift in a single tutorial page.
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: mpsc::Sender<Arc<Task>>,
}

impl Task {
    // for the task schedule it send the task to MiniToki to check the poll
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone());
        // this is hook up with MiniTokio scheduler to execute the task and send by ArcWake with Sync for this purpose
        // &Arc<Self> is cloned in self.executor.send(..)
    }

    fn poll(self: Arc<Self>) {
        // Create a waker from the task instance . this uses the arcwake impl 
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        // No other thread ever tires to lock the future

        let mut  future = self.future.try_lock().unwrap();
        // poll the future
        let _ = future.as_mut().poll(&mut cx);
    }

    fn spawn<F>(future: F, sender: &mpsc::Sender<Arc<Task>>)
    where
        F: Future<Output=()> + Send + 'static ,
        {
            let task = Arc::new(Task {
                future: Mutex::new(Box::pin(future)),
                executor: sender.clone()
            });
            let _ = sender.send(task);
        }
}
// ArcWake implement Send + Sync marker trait
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule()
    }
}

impl MiniTokio {
    fn new() -> MiniTokio {
        let (sender, scheduled) = mpsc::channel();
        MiniTokio { scheduled, sender }
    }

    fn run(&self) {
        while let Ok(task) = self.scheduled.recv()  {
            task.poll();
        }
    }
    // spwan is method of minitoki that take future as argument future will be a future and implemnt Send trait and static
    // for thread safety and lifetime will be entirer programm.

    fn spwan<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender) ;
    }
    // fn run(&mut self) {
    //     let waker = task::noop_waker();
    //     let mut cx = Context::from_waker(&waker);
    //     while let Some(mut task) = self.tasks.pop_front() {
    //         if task.as_mut().poll(&mut cx).is_pending() {
    //             self.tasks.push_back(task);
    //         }
    //     }
    // }
}
