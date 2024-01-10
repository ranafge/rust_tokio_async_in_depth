use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

// A basic implementation of future with stuct

struct Delay {
    when: Instant
}

//Future implementation
impl Future for Delay {
    type Output = &'static str;
    
    /*
        a future must have an `output` type
        what that means is when we resolve (usually await) the future, we need to know what type of future will return
        Kind of like, I'm going to cook, this is the type of food you'll get like indian, thai, chines etc
    
    */
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")

        }else {
            // Ignore this line for now
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}



#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    
    let future = Delay {when};
    let out = future.await;
    /*
     * await, internally, does the following
     * It continuously calls the poll() function inside of the "future" variable
     * It does not exactly do this using a loop
     * Instead, it calls poll once, and if it gets a Poll::Pending back, it yields controll of the executor
     * The executor then russ other things, and comes back and checks on (calls) the poll function again later
     * Thus we achieve concurrency
     */
    assert_eq!(out, "done");
    //resolving the future should return "done", as specified in the poll() function.
}
