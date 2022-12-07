use crate::currencies::{CryptoCurrency, Currency};
use CryptoCurrency::*;

enum APIVersion {
    V3,
}

type URLString = String;

pub const API_BASE_URL: &str = "https://api.binance.us";

fn get_api_version_string(version: APIVersion) -> String {
    match version {
        APIVersion::V3 => String::from("v3"),
    }
}

pub fn get_currency_string_for_url(currency: Currency) -> URLString {
    match currency {
        Currency::CryptoCurrency(USDT) => String::from("USDT"),
        Currency::CryptoCurrency(BTC) => String::from("BTC"),
        _ => panic!("Currency not supported"),
    }
}

pub fn get_latest_price_url(currency: Currency) -> URLString {
    return format!(
        "{}/api/{}/ticker?symbol={}{}",
        API_BASE_URL,
        get_api_version_string(APIVersion::V3),
        get_currency_string_for_url(Currency::CryptoCurrency(CryptoCurrency::BTC)),
        get_currency_string_for_url(currency)
    );
}
