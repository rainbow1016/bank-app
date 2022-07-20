use tide::Request;
use tide::prelude::*;
use log::{info};

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    info!("Starting Tide Server");
    let mut app = tide::new();
    app.at("/orders/shoes").get(order_shoes);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    Ok(format!("Hello, {}! I've put in an order for {} shoes", "name", "legs").into())
}
