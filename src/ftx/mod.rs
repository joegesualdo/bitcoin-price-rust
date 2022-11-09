use anyhow::{Ok, Result};
mod api;

pub type Price = f32;

pub fn get_last_price() -> Result<Price> {
    let response: api::FTXTradesResponse = api::request_trades_data()?;
    let price: Price = response.result[0].price;
    return Ok(price);
}
