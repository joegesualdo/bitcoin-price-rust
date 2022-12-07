//! # Bitcoin Price
//!
//! `bitcoin_price` is an easy way to get the current market data for bitcoin.

use anyhow::Result;

mod binance;
mod crypto_dot_com;
mod currencies;
mod kraken;
mod request;

// From: https://benjaminbrandt.com/averages-in-rust/
fn get_mean(list: &[f32]) -> f64 {
    let sum: f32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}

pub fn get_coinbase_price() -> Result<f32> {
    let coinbase_price = coinbase_bitcoin::get_price_data()?;
    Ok(coinbase_price.spot)
}

pub fn get_coinbase_buy_price() -> Result<f32> {
    let coinbase_price = coinbase_bitcoin::get_price_data()?;
    Ok(coinbase_price.buy)
}

pub fn get_coinbase_sell_price() -> Result<f32> {
    let coinbase_price = coinbase_bitcoin::get_price_data()?;
    Ok(coinbase_price.sell)
}

pub fn get_kraken_price() -> Result<f32> {
    kraken::get_spot_price()
}

pub fn get_binance_price() -> Result<f32> {
    binance::get_latest_price()
}

pub fn get_crypto_dot_com_price() -> Result<f32> {
    crypto_dot_com::get_price()
}

/// This gets the average spot price across multiple exchanges
///
/// # Example:
/// ```
/// let price = bitcoin_price::get_average_exachange_spot_price();
/// assert_eq!(1,1);
/// ```
pub fn get_average_exchange_spot_price() -> f64 {
    let mut list: Vec<f32> = Vec::new();

    match coinbase_bitcoin::get_spot_price() {
        Ok(price) => list.push(price),
        Err(_err) => {}
    }
    match kraken::get_spot_price() {
        Ok(price) => list.push(price),
        Err(_err) => {}
    }
    match binance::get_latest_price() {
        Ok(price) => list.push(price),
        Err(_err) => {}
    }
    match crypto_dot_com::get_price() {
        Ok(price) => list.push(price),
        Err(_err) => {}
    }

    let average_price: f64 = get_mean(&list);
    average_price
}
