use load_balancer_in_rust::{LoadBalancer, Strategy};

#[tokio::main]
async fn main() {
    LoadBalancer::new(
        9090,
        Strategy::RoundRobin,
        &[
            "127.0.0.1:8080",
            "127.0.0.1:8081",
            "127.0.0.1:8082",
            "127.0.0.1:8083",
        ],
    )
    .start().await;
}
