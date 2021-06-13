use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::{delay_for, Delay};

fn sleep(milliseconds: u64) -> Delay {
    delay_for(Duration::from_millis(milliseconds))
}

#[derive(Debug)]
enum Message {
    Hello,
}

async fn message_generator(mut channel: Sender<Message>) {
    loop {
        match channel.send(Message::Hello).await {
            Ok(()) => sleep(100).await,
            Err(_) => {
                eprintln!("Error sending message.");
                break;
            }
        }
    }
}

async fn file_sink(mut channel: Receiver<Message>) {
    while let Some(message) = channel.recv().await {
        println!("Writing to file: {:?}", message)
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = channel::<Message>(10);

    // message_generator -> file_sink
    tokio::spawn(message_generator(tx));
    tokio::spawn(file_sink(rx));

    println!("\nHello World");
    sleep(1000).await;
    println!("\nHello World")
}
