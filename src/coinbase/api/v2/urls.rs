use crate::currencies::{Currency, CryptoCurrency, FiatCurrency};
use CryptoCurrency::*;
use FiatCurrency::*;

enum APIVersion {
    V2,
}

type URLString = String;

pub const API_BASE_URL: &str = "https://api.coinbase.com";

fn get_api_version_string(version: APIVersion) -> String {
    match version {
        APIVersion::V2 => String::from("v2")
    }
}

pub fn get_currency_string_for_url(currency: Currency) -> URLString {
    match currency {
        Currency::FiatCurrency(USD) => String::from("USD"),
        Currency::CryptoCurrency(USDT) => String::from("USDT"),
        Currency::CryptoCurrency(BTC) => String::from("BTC")
    }
}


pub fn get_spot_price_url(currency: Currency) -> URLString {
    return format!(
        "{}/{}/prices/{}-{}/spot",
        API_BASE_URL,
        get_api_version_string(APIVersion::V2),
        get_currency_string_for_url(Currency::CryptoCurrency(CryptoCurrency::BTC)),
        get_currency_string_for_url(currency)
    );
}

pub fn get_buy_price_url(currency: Currency) -> URLString {
    return format!(
        "{}/{}/prices/{}-{}/buy",
        API_BASE_URL,
        get_api_version_string(APIVersion::V2),
        get_currency_string_for_url(Currency::CryptoCurrency(CryptoCurrency::BTC)),
        get_currency_string_for_url(currency)
    );
}

pub fn get_sell_price_url(currency: Currency) -> String {
    return format!(
        "{}/{}/prices/{}-{}/sell",
        API_BASE_URL,
        get_api_version_string(APIVersion::V2),
        get_currency_string_for_url(Currency::CryptoCurrency(CryptoCurrency::BTC)),
        get_currency_string_for_url(currency)
    );
}


