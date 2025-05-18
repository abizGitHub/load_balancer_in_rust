use std::{io, thread};
use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream},
};

#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
async fn main() -> io::Result<()> {
    const LISTENING_PORT: u16 = 9090; // Listening port
    const TARGET_PORT: u16 = 3000; // Target port

    let listener = TcpListener::bind(("127.0.0.1", LISTENING_PORT)).await?;
    println!(
        "Forwarding localhost:{} → localhost:{}",
        LISTENING_PORT, TARGET_PORT
    );

    loop {
        let (mut inbound, addr) = listener.accept().await?;
        println!("{:?} New connection from {}", thread::current().id(), addr);

        tokio::spawn(async move {
            match TcpStream::connect(("127.0.0.1", TARGET_PORT)).await {
                Ok(mut outbound) => {
                    println!("{:?} Connection {} came!", thread::current().id(), addr);
                    let _ = copy_bidirectional(&mut inbound, &mut outbound).await;
                    println!("{:?} Connection {} closed", thread::current().id(), addr);
                }
                Err(e) => {
                    eprintln!("Failed to connect to port {}: {}", TARGET_PORT, e);
                }
            }
        });
    }
}
