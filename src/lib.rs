mod coinbase;

pub fn get_coinbase_price() -> f32 {
    return coinbase::get_spot_price();
}
