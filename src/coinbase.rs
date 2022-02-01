// Source: https://developers.coinbase.com/docs/wallet/guides/price-data
use serde::Deserialize;

#[path = "./request.rs"]
mod request;

const API_BASE_URL: &str = "https://api.coinbase.com";

#[derive(Debug, Deserialize)]
struct BaseResponse {
    base: String,
    currency: String,
    amount: String,
}

#[derive(Debug, Deserialize)]
struct CoinbaseResponse {
    data: BaseResponse,
}

enum APIVersion {
    V2,
}

enum Currency {
    USD,
}

fn get_price_from_response(response: CoinbaseResponse) -> f32 {
    // Expected Response:
    //  {
    //    "data": {
    //      "base":"BTC",
    //      "currency":"USD",
    //      "amount":"39865.46"
    //    }
    //  }
    let price: f32 = 
        response
        .data
        .amount
        .parse()
        .unwrap();
    return price;
}


fn get_api_version_string(version: APIVersion) -> String {
    match version {
        APIVersion::V2 => String::from("v2")
    }
}

fn get_currency_string(currency: Currency) -> String {
    match currency {
        Currency::USD => String::from("USD")
    }
}

fn get_spot_price_url(version: APIVersion, currency: Currency) -> String {
    return format!(
        "{}/{}/prices/BTC-{}/spot",
        API_BASE_URL,
        get_api_version_string(version),
        get_currency_string(currency)
    );
}

fn get_buy_price_url(version: APIVersion, currency: Currency) -> String {
    return format!(
        "{}/{}/prices/BTC-{}/buy",
        API_BASE_URL,
        get_api_version_string(version),
        get_currency_string(currency)
    );
}

fn get_sell_price_url(version: APIVersion, currency: Currency) -> String {
    return format!(
        "{}/{}/prices/BTC-{}/sell",
        API_BASE_URL,
        get_api_version_string(version),
        get_currency_string(currency)
    );
}


pub fn get_spot_price() -> f32 {
    let version: APIVersion = APIVersion::V2;
    let currency: Currency = Currency::USD;
    let request_url: String = get_spot_price_url(version, currency);
    let response: CoinbaseResponse = request::request(request_url);
    let price: f32 = get_price_from_response(response);
    return price;
}

pub fn get_buy_price() -> f32 {
    let version: APIVersion = APIVersion::V2;
    let currency: Currency = Currency::USD;
    let request_url: String = get_buy_price_url(version, currency);
    let response: CoinbaseResponse = request::request(request_url);
    let price: f32 = get_price_from_response(response);
    return price;
}

pub fn get_sell_price() -> f32 {
    let version: APIVersion = APIVersion::V2;
    let currency: Currency = Currency::USD;
    let request_url: String = get_sell_price_url(version, currency);
    let response: CoinbaseResponse = request::request(request_url);
    let price: f32 = get_price_from_response(response);
    return price;
}
