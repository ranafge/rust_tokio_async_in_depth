// One of way to concurrency is spawn a new task
// Here we will define more way to gain concurrencey in this progamm
// tokio::select!

// tokio::select! allow wating on mutiple async computation and return when a single computation completes.

use tokio::sync::oneshot;
async fn some_operation() -> String{}

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        tokio::select! {
            val = some_operation() => {

                let _= tx1.send(val);
            }
            _ = tx1.closed() => {
                // Some operation is cancell, the task complete and tx1 is dropped.
            }
        }
    });
    tokio::spawn(async {
        let _= tx2.send("two");
    });

    tokio::select! {
        // the branch does not complet is droped. the oneshort::Receiver for the channel that did not complete yet is dropped.
        val = rx1 => {
            println!("rx1 is completed first with {:?}", val);
        }
        val = rx2 =>{
            println!("rx2 is completed first with {:?}", val)
        }
        // drop or cancellation performed by dropping a future. if future is drop the operation cann't proceed because all associated state has been droped.
        // drop is impleted for cleaning background resources. tokio onshort::Receiver implements drop trait by send the close notification to the sender.
        // then send receive close notification and proceed to droping operation.

    }
}