async fn action() {
    // some async operation
}

use tokio::sync::mpsc;

#[tokio::main]

async fn main() {
    let (mut tx, mut rx) = mpsc::channel(128);
    let operation = action();
    tokio::pin!(operation);
    // Doing an operation and keen an eye on it tokio::pin!(operation);
    // checking msg for letter rx.recv()
    // if finished the operations no need to check the message and loop break
    // if operation is on going  , recevie the even number message then stop operations.
    loop {
        tokio::select! {
            _=&mut operation => break,
            Some(v) = rx.recv() => {
                if v % 2 ==0 {
                    break;
                }
            }
        }
    }
}