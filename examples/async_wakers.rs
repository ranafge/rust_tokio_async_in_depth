// Wakers section

use  std::future::Future;
use std::pin::Pin;
use std::task::{Context,Poll};
use std::thread;
use std::time::Instant;

// Update Delay to use wakers



struct Delay {
    when: Instant
}

impl Future for Delay {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        }else{
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            let when = self.when;

            // spawn a timer thread
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }
                // waker signal to the executor check the poll state is ready 
                waker.wake();
            });
            Poll::Pending
        }
    }
}

fn main() {
    
}