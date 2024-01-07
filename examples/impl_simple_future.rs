use std::future::Future;
// Pin from standard library
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

// struct
struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("{:?}{:?}", Instant::now(), self.when);
        if Instant::now() >= self.when {
            // here poll method is calling continusly untill meet the condition . if meet the conditon the return poll::ready or else
            println!("hello world");
            Poll::Ready("done")
        } else {
            // at this point future calll continusly pending. this is not very efficient and this process cup cycles
            // for this reason we want mini tokio for only poll future when the future is able to make progress.
            // when a task want read data from a TCP Socket, then we want to poll the task when TCP socket has received data.
            // to achive this, if send a notification that the poll is ready to read of future ready.
            cx.waker().wake_by_ref();
            // waker() method when called singnal to the executor to notify the state is ready to poll
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let delay = Delay { when };
    // delay is the caller or owner of the poll mehtod. it continiusly being calling poll mehtod.
    // wait until resolve the future
    let out = delay.await;
    assert_eq!(out, "done");
}
