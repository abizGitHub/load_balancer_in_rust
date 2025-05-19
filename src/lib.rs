use rand::random_range;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::io;
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};

pub struct LoadBalancer {
    listening_port: u16,
    strategy: Strategy,
    servers: Vec<String>,
}
pub enum Strategy {
    Random,
    RoundRobin,
}

trait BalancingStrategy {
    fn take_server(&self) -> &str;
    fn get_server_back(&self, server: &str);
}

struct RandomStrategy {
    servers: Vec<String>,
}

impl RandomStrategy {
    fn new(servers: &[&str]) -> Self {
        RandomStrategy {
            servers: servers.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl BalancingStrategy for RandomStrategy {
    fn take_server(&self) -> &str {
        let random_ix = random_range(..self.servers.len());
        self.servers[random_ix].as_str()
    }

    fn get_server_back(&self, server: &str) {}
}

impl LoadBalancer {
    pub fn new(listening_port: u16, strategy: Strategy, servers: &[&str]) -> Self {
        Self {
            listening_port: listening_port,
            strategy: strategy,
            servers: servers.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn start(&self) -> io::Result<()> {
        let strategy = match self.strategy {
            Strategy::Random => RandomStrategy {
                servers: self.servers.clone(),
            },
            _ => RandomStrategy {
                servers: self.servers.clone(),
            },
        };
        let strategy = Arc::new(strategy);
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(async move {
                let listener = TcpListener::bind(("127.0.0.1", self.listening_port)).await?;
                println!(
                    "Forwarding localhost:{} → {:?}",
                    self.listening_port, self.servers
                );
                loop {
                    let strategy = Arc::clone(&strategy);
                    let (mut inbound, addr) = listener.accept().await?;
                    println!("{:?} New connection from {}", thread::current().id(), addr);

                    tokio::spawn(async move {
                        let server = strategy.take_server();
                        match TcpStream::connect(server).await {
                            Ok(mut outbound) => {
                                println!("{:?} Connection {} came!", thread::current().id(), addr);
                                let _ = copy_bidirectional(&mut inbound, &mut outbound).await;
                                strategy.get_server_back(server);
                                println!("{:?} Connection {} closed", thread::current().id(), addr);
                            }
                            Err(e) => {
                                eprintln!("Failed to connect to server {}: {}", server, e);
                            }
                        }
                    });
                }
            })
    }
}
