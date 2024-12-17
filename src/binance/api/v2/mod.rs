// Source: https://binance-docs.github.io/apidocs/spot/en/#symbol-price-ticker
use crate::currencies::{CryptoCurrency, Currency};
use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;
use CryptoCurrency::*;

mod urls;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BinanceTickerResponse {
    // pub symbol: String,
    // pub price_change: String,
    // pub price_change_percent: String,
    // pub weighted_avg_price: String,
    // pub open_price: String,
    // pub high_price: String,
    // pub low_price: String,
    pub lastPrice: String,
    // pub volume: String,
    // pub quote_volume: String,
    // pub open_time: i32,
    // pub close_time: i32,
    // pub first_id: i32,
    // pub last_id: i32,
    // pub count: i32
}

pub fn request_latest_price() -> Result<BinanceTickerResponse> {
    let currency: Currency = Currency::CryptoCurrency(Usdt);
    let request_url: String = urls::get_latest_price_url(currency);
    let response: BinanceTickerResponse = request::request(request_url)?;
    Ok(response)
}
