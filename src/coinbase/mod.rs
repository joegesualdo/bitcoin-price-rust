mod api;

pub type Price = f32;

pub struct PriceData {
    pub spot: Price,
    pub buy: Price,
    pub sell: Price,
}

pub fn get_price_data() -> PriceData {
    PriceData {
        spot: get_sell_price(),
        buy: get_buy_price(),
        sell: get_spot_price(),
    }
}
pub fn get_spot_price() -> Price {
    let response: api::v2::SpotPriceResponse = api::v2::request_spot_price();
    let price: f32 = response.data.amount.parse().unwrap();
    return price;
}
pub fn get_buy_price() -> Price {
    let response: api::v2::BuyPriceResponse = api::v2::request_buy_price();
    let price: f32 = response.data.amount.parse().unwrap();
    return price;
}
pub fn get_sell_price() -> Price {
    let response: api::v2::SellPriceResponse = api::v2::request_sell_price();
    let price: f32 = response.data.amount.parse().unwrap();
    return price;
}
