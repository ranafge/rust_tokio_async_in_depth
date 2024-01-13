use tokio::net::TcpStream;
use tokio::sync::oneshot;

#[tokio::main]

async fn main() {
    let (tx, rx) = oneshot::channel();

    // spawn a task that send a message for the oneshot
    tokio::spawn(async move {
        tx.send("done").unwrap();
    });
    tokio::select! {
        socket = TcpStream::connect("localhost:3465") => {
            println!("Socket connected {:?}", socket);
        }
        msg = rx => {
            println!("received message first {:?}", msg);
        }
    }
}