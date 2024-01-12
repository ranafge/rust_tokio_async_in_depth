/*
    Implement wake methodology (add the code for when "wake" should be called)
    Note that the below implementation is not fully complete; there are some loose ends that will be covered in loose ends section
*/

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        // cx is something that is intrinsically linked with this task
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            let when = self.when;
            // Spanwn a time thread.
            thread::spawn(move || {
                // note that this is not a tokio green thread; we are not using tokio in these few turorials, we are making our own tokio
                // thread::spawn has the same effect, though it is a lot more memory intensive that using tokio hteads
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                // this checks to see if the time has elapsed
                // if it has not, wait that time with thread::sleep
                waker.wake();
                // regardless, call wake, letting the executor that this function is ready
            });
            Poll::Pending
            /*
                a not from the book
                when a future returns Poll::pending, it must ensure that the waker is signalled at some point. Forgetting to do this
                results in the task hanging indefinitely.
                Forgetting to wake a task after returning Poll::Pending is a common source of bugs
            */
        }
    }
}

fn main() {}
/*
    Note that in our previous implementaton of Future on Delay, we did this:
        else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    cx.waker().wake_by_ref(); essentially calls the walker every time the poll() is about to return pending
    This worked for us then (because we did not know waht it was) but it is not good practice
    essentially it keeps pining the executor ot check on this future whenever it returns peinding
    while the code will still work, it wasted CPU resources
*/
