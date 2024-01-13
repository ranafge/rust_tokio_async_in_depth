/*
    On handaling the probem of moved futures
    The ida is, on each call to poll, the future checks if the supplied waker matches the previously recoreded waker.
    If the two wakers match, then there is nothing to else to do
    If they do not match, then the recorded waker must be updated
*/

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
    // This is Some when we have spawned a thread, and None otherwise.
    waker: Option<Arc<Mutex<Waker>>>,
    //a waker field is added to keep track of Delay's current waker
}

impl Future for Delay {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        // Check the current instant. If the duration has elapsed, then
        // this future has completed so we return Poll::Ready as usual, no change of this getting moved since it it already done
        if Instant::now() >= self.when {
            return Poll::Ready(String::from("Done!"));
        }

        // The duration has not elapsed. If this is the first time the future
        // is called, spawn the time thread. If the time thread is already running, ensure the stored  `Waker` matches the current task's waker.

        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            /*
            this is the current waker inside the Delay instance (field "waker")
            we use this to check against the current waker in the task where Delay is being polled from
                which can be different if Delay has been moved
            As mentioned in the comment, the first time this runs the waker form the

             */
            // Check if the stored waker matches the current task's waker.
            // THis is necessary as the Delay future instance may move to  a different task between calls to `poll`. If this happens, the waker
            // contained by the given `context` will differ and we must update our store waker to reflect this change.

            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();

                /*

                   This checks to see if the waker stored inside of the Delay instance will make the current context waker(
                       meaning is the current waker is Delay linked with the current Context' (which comes form the thread Delay is being polled form ) waker;
                   )
                   Specially it checks if it will NOt wake the current Context waker (!waker)
                   And if it finds out that the current waker in the Delay instace will not wake the current Context' waker, we replace the current waker in
                   Delay with the waker form the context
                   THis is an indication that the Dealy task has moved.
                   This requires a modification of the waker in Delay to make sure we wake Delay properly from the right thread.
                */
                println!("Updating the Waker because the future has moved to a new task.");
                // adding print statement to show that waker was updated because future was moved
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());

            /*
               This handles the first time the function is polled
               waker field is set to None initally (see main function below)
               on the first run of poll(), the above if statement (if let Some(waker) = &self.waker) is false, since the initial value is None

               for this case, the waker for the current context is cloned and input into the `waker` field in Delay
               the next time the function is polled, the above if( if let Some(waker) = &self.waker) is true, and will compare the waker in Delay with the wake in the current context
            */

            //This is the first time `poll` is called, spawn the timer thread.
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                //The duration has elapsed. Notify the caller by invoking the waker

                let waker = waker.lock().unwrap();
                println!("Waking up the task because the duration has elapsed.");
                waker.wake_by_ref();
            });
        }

        // By now, the waker is stored and timer thread is started.
        // The duration has not elapsed so future has not completed so return Poll::Pending

        // The Future trait contract requires that then pending is return, the future ensures that the given waker is signalled
        //once the future should be polled again. In our case, by retuning Pending here.

        // if we forget to invoke the waker , the task will hang indefinitely.
        Poll::Pending
    }
}

use futures::future::poll_fn;

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let mut delay = Some(Delay {
        when,
        waker: None, // initally None, this will filled when the function is polled at lease once
    });

    poll_fn(move |cx| {
        let mut delay = delay.take().unwrap();
        let res = Pin::new(&mut delay).poll(cx);
        assert!(res.is_pending());
        println!("Is pending");
        tokio::spawn(async move {
            println!("{}", delay.await);
        });
        Poll::Ready(())
    })
    .await;
    std::thread::sleep(Duration::from_secs(5));
}
