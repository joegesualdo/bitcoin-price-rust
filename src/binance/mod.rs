use anyhow::{Ok, Result};
mod api;

pub type Price = f32;

pub fn get_latest_price() -> Result<Price> {
    let response: api::v2::BinanceTickerResponse = api::v2::request_latest_price()?;
    let price: f32 = response.lastPrice.parse()?;
    return Ok(price);
}
