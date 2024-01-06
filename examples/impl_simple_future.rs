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
        if Instant::now() >= self.when { // here poll method is calling continusly untill meet the condition . if meet the conditon the return poll::ready or else
            println!("hello world");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
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
