mod balancing_strategy;

use std::error::Error;
use std::future::Future;
use std::sync::Arc;
//use std::sync::{Arc};
use std::thread;
use tokio::io;
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub struct LoadBalancer {
    listening_port: u16,
    strategy: Strategy,
    servers: Vec<String>,
}
pub enum Strategy {
    Random,
    RoundRobin,
}
trait BalancingStrategy: Send {
    fn take_server(&mut self) -> &str;
    fn get_server_back(&mut self, server: &str);
}
struct RandomStrategy {
    servers: Vec<String>,
}
struct RoundRobinStrategy {
    servers: Vec<String>,
    ix: usize,
}
impl LoadBalancer {
    pub fn new(listening_port: u16, strategy: Strategy, servers: &[&str]) -> Self {
        Self {
            listening_port: listening_port,
            strategy: strategy,
            servers: servers.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub async fn start(&self) -> io::Result<()> {
        let _strategy: Arc<Mutex<dyn BalancingStrategy>> = match self.strategy {
            Strategy::Random => Arc::new(Mutex::new(RandomStrategy {
                servers: self.servers.clone(),
            })),
            Strategy::RoundRobin => Arc::new(Mutex::new(RoundRobinStrategy {
                servers: self.servers.clone(),
                ix: 0,
            })),
            _ => panic!("invalid strategy"),
        };

        let listener = TcpListener::bind(("127.0.0.1", self.listening_port)).await?;
        println!(
            "Forwarding localhost:{} → {:?}",
            self.listening_port, self.servers
        );
        let strategy = Arc::new(Mutex::new("adasd".to_string()));//Arc::clone(&strategy);
        loop {
            let strategy = Arc::clone(&strategy);
            let (mut inbound, addr) = listener.accept().await?;
            println!("{:?} New connection from {}", thread::current().id(), addr);
            tokio::spawn(async move {
                //let estrategy = Arc::clone(&strategy);
                let mut binding = strategy.lock().await;
                let server = binding.to_string();//take_server();
                match TcpStream::connect(binding.to_string()).await {
                    Ok(mut outbound) => {
                        println!("{:?} Connection {} came!", thread::current().id(), addr);
                        let _ = copy_bidirectional(&mut inbound, &mut outbound).await;
                        //strategy.lock().unwrap().get_server_back(server);
                        println!("{:?} Connection {} closed", thread::current().id(), addr);
                    }
                    Err(e) => {
                        //eprintln!("Failed to connect to server {}: {}", server, e);
                    }
                };

            });
        }
    }
}
