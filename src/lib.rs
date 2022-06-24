//! # Bitcoin Price
//!
//! `bitcoin_price` is an easy way to get the current market data for bitcoin.

use coinbase_bitcoin;

mod request;
mod currencies;
mod kraken;
mod ftx;
mod binance;
mod crypto_dot_com;

// From: https://benjaminbrandt.com/averages-in-rust/
fn get_mean(list: &[f32]) -> f64 {
    let sum: f32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}

pub fn get_coinbase_price() -> f32 {
    return coinbase_bitcoin::get_price_data().spot;
}

pub fn get_coinbase_buy_price() -> f32 {
    return coinbase_bitcoin::get_price_data().buy;
}

pub fn get_coinbase_sell_price() -> f32 {
    return coinbase_bitcoin::get_price_data().sell;
}

pub fn get_kraken_price() -> f32 {
    return kraken::get_spot_price();
}
pub fn get_ftx_price() -> f32 {
    return ftx::get_last_price();
}

pub fn get_binance_price() -> f32 {
    return binance::get_latest_price();
}
pub fn get_crypto_dot_com_price() -> f32 {
    return crypto_dot_com::get_price();
}

/// This gets the average spot price across multiple exchanges
///
/// # Example:
/// ```
/// let price = bitcoin_price::get_average_exachange_spot_price();
/// assert_eq!(1,1);
/// ```
pub fn get_average_exchange_spot_price() -> f64 {
    let list = vec![
        coinbase_bitcoin::get_spot_price(),
        kraken::get_spot_price(),
        ftx::get_last_price(),
        binance::get_latest_price(),
        crypto_dot_com::get_price(),
    ];
    let average_price: f64 = get_mean(&list);
    return average_price;
}
