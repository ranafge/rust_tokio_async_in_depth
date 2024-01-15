use tokio_stream::StreamExt;
use mini_redis::client::{self, Message};

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    // client connect to the redis server using the ip and port nubmer.

    // Publish some data

    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "2".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    // numbers is the name of the channel sending 1, 2,3 four so on.

    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    // coneect to the mini redis server 
    let subscribe = client.subscribe(vec!["numbers".to_string()]).await?;
    // subscribe the numbers channel.
    let messages = subscribe.into_stream();
    // convert he numbers channel message to inot_stream
    // let messages = subscribe.into_stream().filter(|msg| match  msg {
    //     Ok(msg) if msg.content.len() == 1 => true,
    //     _=>false
        
    // }).take(3);
    tokio::pin!(messages);
    // Before the work with message we need to pin it. so the message cann't move in the memory
    // println!("{:?}", messages);
    while let Some(msg) = messages.next().await  {
        println!("GOT ={:?}", msg);
    }


    Ok(())


}

#[tokio::main]
async fn main() -> mini_redis::Result<()>{
    tokio::spawn(async {
        publish().await;
    } );
    subscribe().await?;
    println!("DONE");
    Ok(())
}