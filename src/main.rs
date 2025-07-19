mod trade;
mod ibapi;


use time::macros::datetime;
use ibapi::prelude::*;

#[tokio::main]
async fn main() {
    let connection_url = "127.0.0.1:4002";
    let client = Client::connect(connection_url, 100).await.expect("connection to TWS failed!");

    let contract = Contract::stock("AAPL");

    let historical_data = client
        .historical_data(
            &contract,
            Some(datetime!(2023-04-11 20:00 UTC)),
            1.days(),
            HistoricalBarSize::Hour,
            HistoricalWhatToShow::Trades,
            true,
        )
        .await
        .expect("historical data request failed");

    println!("start: {:?}, end: {:?}", historical_data.start, historical_data.end);

    for bar in &historical_data.bars {
        println!("{bar:?}");
    }
}