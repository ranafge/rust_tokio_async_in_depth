// a stream is an asynchronoush series of values. equivalent to std::iter::Iterator
//Iteration

// rust programming language does not support for loop for asnchronouch that replace with while let 

use tokio_stream::StreamExt;

#[tokio::main]

async fn main() {
    let mut stream = tokio_stream::iter(&[1,2,3]);
    while let Some(v) = stream.next().await {
        // next() return Option<t> t is the steam value type. When None return the stream iteration is teminated.
        println!("GOT = {:?}", v);
    }
}