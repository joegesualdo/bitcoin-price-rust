mod Coinbase;

pub fn get_coinbase_price() -> f32 {
	    return Coinbase::get_spot_price();
}
