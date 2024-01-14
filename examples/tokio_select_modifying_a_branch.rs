async fn action(input: Option<i32>) -> Option<String> {
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    // async logic here to use i value
}

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);
    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });



    loop {
        tokio::select! {
            res = &mut operation, if !done =>{
                done=true;
                if let Some(v) = res {
                    println!("Got ={}", v);
                    return;
                }
            }
            Some(v) = rx.recv() => {
                if v % 2 == 0{
                    operation.set(action(Some(v)));
                    done = false
                }
            }
        }
    }
}
/*
Imagine you are a chef who needs to make sandwiches. While making sandwiches, you also need to keep an eye on your mailbox for special letters. Here's what's happening:

Making Sandwiches:

You start making a sandwich (action(None)).
You keep an eye on the sandwich-making process (tokio::pin!(operation)).
Checking Mailbox:

At the same time, you check your mailbox for letters (rx.recv()).
If someone sends you an even-numbered letter, you want to pause making the current sandwich and start making a new sandwich with the even number.
Loop Logic:

You're in a loop doing these two tasks repeatedly.
If the sandwich is ready, you print what you got and finish.
If you receive a new even number in the mailbox, you stop the current sandwich-making and start a new one with the even number.
Precondition (if !done):

This means, "Only pause making the current sandwich if it's not already done."
It's like saying, "Don't interrupt if you're already done with the sandwich."
Avoiding Panic:

Without checking if the sandwich task is done (if !done), there could be a problem.
The done variable keeps track of whether you've finished making the current sandwich or not.
In simple terms, you're multitasking: making sandwiches and checking your mailbox. If a new even-numbered letter arrives, you might stop making the current sandwich, but only if you haven't finished it yet. This way, everything happens in a coordinated manner, avoiding issues.



*/