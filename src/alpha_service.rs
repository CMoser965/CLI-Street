use alphavantage::{time_series::TimeSeries, Client};
use dotenv::dotenv;

struct StockMarket {
    av_client: Client,
}

impl StockMarket {
    fn new<T: StockMarketInit>(init: T) -> Self {
        init.create_stock_market()
    }

    
}

pub trait StockMarketInit {
    fn create_stock_market(self) -> StockMarket;
}

impl StockMarketInit for Client {
    fn create_stock_market(self) -> StockMarket {
        let api_token = std::env::var("AV_KEY");
        let client = Client::new(&api_token.unwrap());
        StockMarket { av_client: client }
    }
}