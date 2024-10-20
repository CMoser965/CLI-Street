mod alpha_service;

use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_token = std::env::var("AV_KEY");
    let client = Client::new(&api_token.unwrap());
    let time_series = client.get_time_series_daily("GOOG").await.unwrap();
    let entry = time_series.entries.last().unwrap();
    println!("{:?}", entry);

    let exchange_rate = client.get_exchange_rate("USD", "EUR").await.unwrap();
    println!("{:?}", exchange_rate);
}