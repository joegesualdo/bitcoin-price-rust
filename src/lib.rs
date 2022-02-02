//! # Bitcoin Price
//!
//! `bitcoin_price` is an easy way to get the current market data for bitcoin.

mod request;
mod currencies;
mod coinbase;
mod kraken;

pub fn get_coinbase_price() -> f32 {
    return coinbase::get_price_data().spot;
}

pub fn get_coinbase_buy_price() -> f32 {
    return coinbase::get_price_data().buy;
}

pub fn get_coinbase_sell_price() -> f32 {
    return coinbase::get_price_data().sell;
}

pub fn get_kraken_price() -> f32 {
    return kraken::get_spot_price();
}

/// This gets the average spot price across multiple exchanges
///
/// # Example:
/// ```
/// let price = bitcoin_price::get_average_exachange_spot_price();
/// assert_eq!(1,1);
/// ```
pub fn get_average_exachange_spot_price() -> f32 {
    let coinbase_price: f32 = coinbase::get_spot_price();
    let kraken_price: f32 = kraken::get_spot_price();
    let average_price: f32 = (coinbase_price + kraken_price) / 2.0;
    return average_price;
}
