use futures::task;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
//import from here: https://crates:io/crates/futures

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world!");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
/*
    Implementing future trait on Delay, like we did before

*/

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;
/*
    Specifying what an asynchronous task truly is
    Pin is complicated. Understand that its job is to ensure  a future is not moved as a prerequisite to being polled
    We ensure all tasks are Futures with Box<dyn Future
    The output of the future is our particular example is () (example tuple , returns nothing)
    So essentially a Task is a future that executes but returns nothing
    I think it is just used for exaple purpose here
    Must implemnet the send trait
*/
struct MiniTokio {
    tasks: VecDeque<Task>,
    /*
       MiniTokio is an executor (runtime environment), a sized down version of Tokio
       Here, we add one field, "task" which stores all the tasks that the executor must execute
    */
}

impl MiniTokio {
    fn new() -> MiniTokio {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }

    // Spwan a future onto the mini-tokio instance
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
        // The spawn function is used to add tasks of the type Task, specified above to the MiniTokio queue for execution
        // push_back adds them to the end of the queue
    }
    fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        // explained later in waker sections
        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task)
            }
        }

        /*
           This loops through the queue of tasks in the tasks field
           It first takes the first task in the queue using self.tasks.pop_front()
           ** the task is polled task.as_mut().poll(&mut cx) **
               and the return value is checked, to see if it is pending --> is_pending()
           if it is pending, add back to the queue, at the backof the queue
           It not we specified in the definition of Task that futures added to the queue won't return anything (Output=())

           while this is great, it does have a problem
           The loop will just continue to run forever and ever  until all the futures inside of it are all resolved.
           Think of the analogy of a baker who has a queue of ovens (tasks) and he constantly goes from oven to oven to see if the food inside is down
           He's gonna get fried
           It's the same with a computer; it won't necessarily get tried but it will consume resources, having to constantly check each future
           It would be a lot more convenient if each oven had some way to signal the baker, like a ding sound, or a "waker" that lets the baker know that the food is ready,
           So the doesn't have to constantly check each oven
        */
    }
}
fn main() {}
