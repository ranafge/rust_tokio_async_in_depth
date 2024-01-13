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
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let when = self.when;
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now)
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}

use futures::future::poll_fn;
#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let mut delay = Some(Delay { when });
    poll_fn(move |cx| {
        // Poll_fn creates a new future on the spot from a clouse
        // context comes from the executor (main thread in this case tokio::main) that is executing
        // one context/waker passed here

        let mut delay = delay.take().unwrap();
        let res = Pin::new(&mut delay).poll(cx);
        // delay is polled here and partially executed i.e it be cooking in one oven

        assert!(res.is_pending());

        tokio::spawn(async move {
            delay.await;
            // now delay is moved to another oven and cooked (its execution and continued and finished)here;
            // another Context/warker passed here
        });
        Poll::Ready(())
    })
    .await;
}

/*
    Future Migration: In rust, a signal future can move accross different tasks while it's execuing.
    This means that the Future can start executing in one task and then continueing in another task.
    This is what's happening in the provided code: The Delay future is first polled in the main task, and then it's moved to a new task
    Whee it's awaited.

    poll_fn funciton: poll_fn function is a helper functin from the future crate that creates a Future from a clouser.
    The clouser takes a Context and returns Poll::Ready(()) when it is doen. In the provided code poll_fn cluse polls the Delay future once, then move the Delay future to a new task.

    In the case of poll_fn, the executor is tokio::main which is a tokio runtime configured to run untill all spawned tasks have completed.
    When tokio::main polls the Future created by poll_fn, it passes in a Context which poll_fn then passes to its cluser.
    Meaning tokio::main gives the context i.e context comes from the async fn main itself (context comes from the executor, in
    this case that is the main thread so it provides the context here for the waker)
    Remember that we need to know the context of each waker to know to know what future to execute when the waker is called
    (5 ovens are cooking, one of them makes an alarm saying they're done. which one made the alarm? Identifying this would
    prove difficult without an identifier of some kind. this is the job of Context)

    Waker instance: The waker is used to wake up a task when a Future is ready to make progress. Each call to poll is passed a Context, which contains a Waker.
    It's important to note that each call to pall could be passed a different Waker. This is because the Future can move to a different task, and each task has it's own waker.
    In the provide code, The Delay future is polled twice in the main task, and once in the new task.
    Each of these polls is passed a different Waker.

    Interesting: So waker are associated with tasks (threads) rather than futures. In the two points . we have different wakers
    So building on the timer analogy with the oven:

        Food (future) in the first oven(main thread), cooks a bit( executes via .await) has one waker ( a time that goes ding)
        Foo (future) is moved to a different oven (spawned task), cooks the rest of the way (.await) has another waker (another timer that goes dong);
        Calling the waker() method will only cause the thread that wake is attached to to be activated(know via context);
        If the future has been moved from there, it will not be executed

        ** updating the waker : when implementing a Future, you must make sure to update any previously recored Warker with
        the new one passed to the most recent call to poll. This is because the Future could have moved to a new task and you need to make sure to wake up the
        currect task.
        when the Future is ready to make progress. if you don't this , you could end up waking up a task that no longer has the Future,
        which would be a wasted of resources.



*/
