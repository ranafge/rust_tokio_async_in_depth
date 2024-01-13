use tokio::sync::Notify;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

async fn delay(dur: Duration) {
    let when = Instant::now() + dur;

    // Notify is a type from the `tokio::sync` module that provides a mechanism to notify a task (thread, executor) when a particlular event has occured.
    /*
        Its job is primarily to abstract away the hassl eof Waker and wake()
        When a future is ready to be executed, simply call notify_one()
     */
    //`Arc` is a type of smart pointer in Rust that allows mutiple thread to have read access to some data and ensure that the data gets cleaned up once all threads are done with it.
    // `Arc::new(Notify::new())` creates a new Notify instace and wraps it in an Arc so it can be shared across threads

    let notify = Arc::new(Notify::new());
    // cloning an `Arc` increases its reference count and allows the clone to be moved to another thread.
    //This clone will be moved into the thread we spawn below.

    let notify_clone = notify.clone();

    // `thread::spawn` starts a new OS thread and runs the provided clouser in it.
    // the  `move` keyword means the clouser takes ownership of the values it uses from the environment, in this case `when` an `notify_clone`.

    thread::spawn(move || {
        let now = Instant::now();

        if now < when {
            thread::sleep(when - now);
        }
        //Once the delay has elapsed, call `notify_one` on the Notify instacne.
        // This wakes up one task that called notifyed().await on the Notify instance.

        notify_clone.notify_one();
        // this essentially does the same thing as wake , but abstracted away.
        //it invokes the stored waker to wake the task
        // this lets whatever executor is handling the "delay" method that the task is ready for execution and it is scheduled
    });

    // notify().await suspends the current task untill the Notify instace is notified.
    // In this case, it will notified when the delay has elapsed.
    notify.notified();
    // Specifically, this waits until notify is "notified", meaning it wait until notify_one is called.
    // Note that notify_one is called in another thread; this thread may finish executing and exit before notify_one has a chance to execute
    //Remember as well that both notify and notify_clone come from the same instace and they are just shared (notify_clone was cloned, )
    //So if we call notify_one on notify_clone, it affects notify as well since the 2 are linked.
}

#[tokio::main]

async fn main() {
    let when = Duration::from_millis(200);
    let delay = delay(when);
    let task = tokio::spawn(async move  {
        delay.await;
        println!("Delay completed");
    });
    task.await.unwrap();

}