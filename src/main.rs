// tokio is a runtime for asyncronous program in rust with out blocking each other.
// net is the submodule of tokio for networking functionality. this is represent a TCP stream channel for
// communication between two entities over TCP a connection
use tokio::net::TcpStream;

async fn my_async_fn() {
    println!("Hello from async");
    // This is the connection channel between two network entities over a TCP connection using port 8080;
    let _socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    // TcpStream connect is tring to connect with "127.0.0.1:8080" but unable because i am not using server at this moment.

    println!("async TCP operation complet");
}

#[tokio::main]
async fn main() {
    let what_is_this = my_async_fn();
    // nothing has been printined yet. because asyncs are lazy to execute with use .await
    what_is_this.await;
}

use std::pin::Pin;
use std::task::{Context, Poll};
// async function return future. here is the below std future library define.
pub trait Future {
    type Output;
    // owner or caller function use Future::poll to advancing the computation by polling the future. is this task reay? if ready then return unles wait for it.
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
