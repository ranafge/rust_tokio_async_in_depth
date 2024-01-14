use tokio::sync::oneshot;

#[tokio::main]

async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    let(tx3, rx3) = oneshot::channel();
    tokio::spawn(async {
        tx1.send("t");
    });

    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            // if there is no receive then else  block will execute 
            else => {break}
            // loop execute here for the break statemtn.

            // message will check that way no message will be lost
        };
        // 
        println!("Got msg {:?}", msg);
    }
    println!("all channel has been closed.")
}