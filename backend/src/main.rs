use log::info;
use tide::prelude::*;
use tide::Request;

#[tokio::main]
async fn main() -> tide::Result<()> {
    info!("Starting Tide Server");
    let mut app = tide::new();
    app.at("/hello-world").get(hello_world);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn hello_world(mut _req: Request<()>) -> tide::Result {
    Ok(format!("Hello World!").into())
}
