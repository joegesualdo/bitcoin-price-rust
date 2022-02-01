mod coinbase;
mod kraken;

pub fn get_coinbase_price() -> f32 {
    return coinbase::get_spot_price();
}

pub fn get_coinbase_buy_price() -> f32 {
    return coinbase::get_buy_price();
}

pub fn get_coinbase_sell_price() -> f32 {
    return coinbase::get_sell_price();
}

pub fn get_kraken_price() -> f32 {
    return kraken::get_spot_price();
}

pub fn get_average_exachange_spot_price() -> f32 {
    let coinbase_price: f32 = coinbase::get_spot_price();
    let kraken_price: f32 = kraken::get_spot_price();
    let average_price: f32 = (coinbase_price + kraken_price)/2.0;
    return  average_price;
}
