mod coinbase;
mod kraken;

pub fn get_coinbase_price() -> f32 {
    return coinbase::get_spot_price();
}
pub fn get_kraken_price() -> f32 {
    return kraken::get_spot_price();
}
