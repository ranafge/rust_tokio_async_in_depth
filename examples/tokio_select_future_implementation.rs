// Hypotheticall implementation of select how its work
use tokio::sync::oneshot;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MySelect{
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>
}

impl Future for MySelect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx){
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(());
        }
        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(());
        }
        Poll::Pending
    }
}

#[tokio::main]

async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    MySelect{
        rx1, rx2
    }.await
    // in MySelect contain two branch that are future if one branch is complete then drop. 
    //If other branch is not complete the operation is efectively cancelled.
    // Select has a limit branches . it it 64 only

    // select! structure pattern is  <pattern> = async expression => handler * here pattern is variabl name
    // when select! is evaluated all async exprestion executed concurrently
    
}