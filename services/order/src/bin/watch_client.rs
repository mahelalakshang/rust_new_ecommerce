//! Demo client for the `WatchOrderStatus` streaming RPC.
//! Usage: cargo run -p order --bin watch_client -- <order_id>

use order::proto::order::{order_service_client::OrderServiceClient, WatchOrderStatusRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let order_id = std::env::args()
        .nth(1)
        .expect("usage: watch_client <order_id>");

    let mut client = OrderServiceClient::connect("http://localhost:50052").await?;

    let mut stream = client
        .watch_order_status(WatchOrderStatusRequest {
            order_id: order_id.clone(),
        })
        .await?
        .into_inner();

    println!("watching order {order_id}...");
    while let Some(update) = stream.message().await? {
        println!("[{}] status = {}", update.updated_at, update.status);
    }
    println!("stream closed");

    Ok(())
}
