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
    let response: api::SpotPriceResponse = api::request_spot_price();
    let price: f32 = 
        response
        .data
        .amount
        .parse()
        .unwrap();
    return price;
}
pub fn get_buy_price() -> Price {
    let response: api::BuyPriceResponse = api::request_buy_price();
    let price: f32 = 
        response
        .data
        .amount
        .parse()
        .unwrap();
    return price;
}
pub fn get_sell_price() -> Price {
    let response: api::SellPriceResponse = api::request_sell_price();
    let price: f32 = 
        response
        .data
        .amount
        .parse()
        .unwrap();
    return price;
}
