use tokio::net::TcpListener;
use tokio::sync::oneshot;
use std::io;


#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send(()).unwrap()
    });
    let mut listener = TcpListener::bind("localhost:3465").await?;
    tokio::select! {
        _= async {
            loop {
                let (socket, _) = listener.accept().await?;
                tokio::spawn(async move {process(socket)});
            }
            Ok::<_, io::Error(())>
        }=> {}

        _= rx => {
            println!("terminationg accept loop")
        }
    }
    Ok(())
}