/*
    Must now make it so mini-tokio (which, remember , is our executor ) is notified when waker is called
    A note that tokio already has this programmed within it to do this
    now, we see a (very basic ) implementation of this
*/ 

use std::f32::MIN;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

struct Delay {
    when : Instant
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        }else {
            let waker = cx.waker().clone();
            let when = self.when;

            thread::spawn( move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }
                waker.wake();
                /*
                    Though wake_by_ref method defined in the Task warpper (far) below, the call to wake()
                    here reschedules the task for execution
                    which makes sense at this point because the time is complete, so this future is done
                    Reschedule meaning poll() functions is called again , and this time it will return Poll::Ready , thereby completing
                */ 
            });
            Poll::Pending
        }
    }
}

/*
 Code with the waker implementation on Delay
 A note that Wakers must implement Send and Sync trait ( which they do in almost all instance, unless they have non Send/Sync types in them like RC)
 This is because they will be sent between threads (sent form whererver "Delay" struct is being awaited, to the main thread)

 * 
 */

 use std::sync::Arc;
 use std::sync::{mpsc, Mutex};

 struct MiniTokio {
    scheduled: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,

    /*
        MiniTokio has two fields: secheduled and sender.
        These  are the two halves of a muti-producer, single-consumer channel.
        The sender is used to send tasks (futures wrapped in a Task struct ) to be executed,
        and scheduled is used to receive these tasks
     */
 }

 impl MiniTokio {
     fn new() -> MiniTokio {
        let (sender, scheduled) = mpsc::channel();
         MiniTokio {
            scheduled , sender
         }
     }
    /*
        the new function creates a new MiniTokio instance.
        It creates a new mpsc channel and assigins the sender and receiver to the sender and scheduled fields, respectively.
     */
    /// Spawn a future onto the mini tokio instance
    /// 
    /// The given future is wrapped with the `Task` harness and pushed into the 
    /// `scheduled` queue. The future will be executed when `run ` is called.
    /// 

     fn spawn<F>(&self, future: F) 
        where
            F: Future<Output = ()> + Send + 'static
        {
            Task::spawn(future, &self.sender);
        }
        /*
            The spawn fuction is used to spawn a new task onto the executor.
            It takes a future, wraps it in a Task struct, and sends it to the sceduled queue via the sender.
            We send a reference to sender (&self.sender) because the Task::spwan method will clone this sender and insert it into the corrrect field in the Task
                Remember, mpsc channels are muti-producer, single consumer
                So each task will have a sender half of the channel which leads back to MiniTokio, which holds the receiver hald in its "scheduled" field
                This mehtod is called in the main function
                The future in this spawn is just a container, that's why it doesn't return anything
                However, for it to resolve, the future within must complete and return someting
                And THis is where we put the Delay instance

                See (far) below in the Task::spawn method and the main method for more details
         */
        fn run(&self) {
            while let Ok(task) = self.scheduled.recv()  {
                task.poll();
            }
        }
        /*
            The run function is the main loop of the executor.
            It continuoulsy receives tasks from the schedule queue and polls them.
                Again, tasks have a sender half which they use to send themselves to mini-tokio
                They are received and processed here via while let
            If a task is not ready yet, its poll mehod will ensure that it gets rescheduled for polling when it becomes ready.
                Task's poll mehtod is below
            
            Unlike before, where tasks were continuously being polled before they were ready task.poll() logic will instead not re send
            the task to mini tokio untill it is ready to proceed
            See below task.poll() mehtod for how        
        
         */
 }

 struct Task {
    // The Mutex is to make Task implement Sync. Only one thread access future at any given time. 
    // The Mutex is not required for correctness. Real tokio does not use a mutext here, bur real tokio has more lines to code than
    // can fit in a single tutorial page.

    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send >>>,
    // task definition (what futures can be put into tasks ) define here
    executor : mpsc::Sender<Arc<Task>> 
 }

/*
 The job of the task struct is to serve as a wrapper around a future
 In this particular case "Delay" struct instances
 The task itself is a wrapper future which resolves when the future it is wrapped around aslo completes
 It is mini-tokios way of tracking when a future is done
 Its like a labelled oven proof smart container that food is put into before being putting into the oven
 it aslo signals to the baker if the food is holds is complete

 A better example might be an apple product
 You can have all of your own data (your own music, pictures, emails, etc.) (futures being inserted into mini-tokio);
 But for it to work on appleas' ecosystem, it must all be placed inside of an apple devide (task wrapper in this case);

 An "executor" field exists as well, which holds a sender hald of an mpsc channel
 The receiver half of this channel is on the receiver field of mini-tokio

*/

use futures::task::{self, ArcWake};
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule()
    }
}

/*
    This is an implementation of the ArcWake trait for Task.
    ArcWake is a trait provided by the futures crate that defines a mehod fo rwaking up a task.
    Here, wake_by_ref is implemented to call the schedule mehtod on Task

    This is waht allows a task to schedule itself once it is ready to proceed
    The logic fo rhow a task schedules itself is refined in the schedule mehtod below
    But in summay, the schedule() mehtod is sends the task to the executor (mini-tokio)

    if you look through the code you will see that wake_by_ref is never explicitly called

    so how does it workd?
    The wake_by_ref mehtod is attached to a waker; when the waker's wake mehtod is called, that's when this 
    mehtod is called too.
    Remember, Task is a wrapper future around an actual future we are trying to resolve
    In this case Delay

    Notice in Delay, we have this code in the else block (executes if the timer is not finished);
    waker.wake();

    Through the code below for Task (in task's impl block) we link Task's wake method with thw wake method of the future it wraps around 
    So essentially what is happening is when the wake method is called in the Delay struct for Poll::Pending, this wake_by_ref method is also called, which reschedules the ask for exeution

*/


impl Task {
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone()).unwrap()
    }
    // As explained above, this re-adds the Task to the executor (mini-tokio)
    // Called when a task was pending but now is ready to proceed (wake() is called on Waker)

    fn poll(self: Arc<Self>) {
        
    }
}