use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};


struct Interval {
    rem: usize,
    delay: Delay
}

impl Interval {
    fn new() -> Self{
        Self {rem: 3, delay: Delay {when: Instant::now()}}
    }
}

pub trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> ;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl Stream for Interval {
    type Item = ();
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<()>> {
        if self.rem  == 0 {
            return Poll::Ready(None)
        }
        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + Duration::from_millis(10);
                self.delay = Delay {when};
                self.rem = 1;
                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending
        }

    }

   
}