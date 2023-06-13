// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=dd18cb2f8a843d9e2fb7800aa7d91c49

extern crate tokio; // 1.17.0
extern crate futures;

use futures::StreamExt;
use futures::stream::{FuturesUnordered};
use tokio::sync::mpsc::{channel};

#[tokio::main]
async fn main() {
    // Start up the replicas
    let replicas = 3;
    let mut host_futures = FuturesUnordered::new();
    let mut host_senders = vec![];
    for host in 0..replicas {
        let (sender, receiver) = channel(2);
        host_senders.push(sender);
        host_futures.push(replica(host, receiver));
    }
    
    // Send the data    
    for message in ['H', 'e', 'l', 'l', 'o', '\n'] {
        for sender in &host_senders {
            sender.send(message).await.unwrap();
        }
    }
    
    while let Some((host, count)) = host_futures.next().await {
        eprintln!("Host {host} received {count} bytes.");
    }
    
    eprintln!("All done")
}

async fn replica(host: u32, mut receiver: tokio::sync::mpsc::Receiver<char>) -> (u32, usize) {
    let mut count = 0;
    while let Some(message) = receiver.recv().await {
        eprintln!("Host {host} received message {message:?}");
        if message == '\n' {
            break;
        } else {
            count += 1;
        }
    }
    (host, count)
}

