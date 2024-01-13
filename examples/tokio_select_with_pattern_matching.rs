use tokio::sync::mpsc;

#[tokio::main]

async fn main(){
    let (mut tx1, mut rx1) = mpsc::channel(128);
    let (mut tx2 , mut rx2) = mpsc::channel(128);
    tokio::spawn(async move {
        tx1.send("one");
    });
    tokio::spawn(async move {
        tx2.send("two");
    });

    tokio::select! {
        Some(v) = rx1.recv() => {
            println!("Got {:?} form rx1 ", v);
        }
        Some(v) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }
        else => {
            println!("Both channels closed");
        }
        // above 2 branch return None the else will be executed
    }
}