use alphavantage::{time_series::TimeSeries, Client};
use dotenv::dotenv;

pub struct StockMarket {
    av_client: Client,
    all_symbols: Vec<String>
}

impl StockMarket {
    pub fn new() -> Self {
        dotenv().ok();
        let api_token = std::env::var("AV_KEY");
        let client = Client::new(&api_token.unwrap());
        StockMarket { av_client: client, all_symbols: vec!["GOOG".to_string()] }
    }

    pub fn add_symbol(&mut self, new_symbol: String) {
        self.all_symbols.push(new_symbol);
    }

    pub async fn get_all_symbols_latest(&self) -> Vec<TimeSeries> {
        let mut time_series: Vec<TimeSeries> = vec![];
        for symbol in self.all_symbols.iter() {
            let temp_time_series = self.av_client.get_time_series_daily_full(symbol).await.unwrap();
            time_series.push(temp_time_series);
        }
        return time_series;
    }
}