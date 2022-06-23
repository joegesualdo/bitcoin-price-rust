// Source; https://exchange-docs.crypto.com/spot/index.html#public-get-ticker
#![allow(dead_code)]

mod api;

type Price = f32;

pub fn get_price() -> Price {
    let response_json: api::CryptoDotComResponse = api::request_ticker_data();
    let price: Price = response_json.result.data.a;
    return price;
}
