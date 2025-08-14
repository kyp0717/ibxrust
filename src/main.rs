use ibapi::prelude::*;

#[tokio::main]
async fn main() {
    let connection_url = "127.0.0.1:4002";

    let client = Client::connect(connection_url, 100)
        .await
        .expect("connection to TWS failed!");
    println!("Successfully connected to TWS at {connection_url}");
}
