mod api;

pub type Price = f32;

pub fn get_last_price() -> Price {
    let response: api::FTXTradesResponse = api::request_trades_data();
    let price: Price = response.result[0].price;
    return price;
}
