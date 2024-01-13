use std::process::Output;

use tokio::sync::oneshot;

#[tokio::main]
async fn main () {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    let mut out = String::new();

    tokio::spawn(
    //    println!("somethig");
    );

    tokio::select! {
        _ = rx1 => {
            out.push_str("rx1 completed");
        }
        _ = rx2 => {
            out.push_str("rx2 complted")
        }
    }
    println!("out is {:?}", out);
    // out that is string that mutablly borrow in select scope
}