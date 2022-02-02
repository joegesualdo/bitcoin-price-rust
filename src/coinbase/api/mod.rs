// Source: https://developers.coinbase.com/docs/wallet/guides/price-data

use serde::Deserialize;
use crate::request;
use crate::currencies::{Currency, CryptoCurrency, FiatCurrency};
use CryptoCurrency::*;
use FiatCurrency::*;

#[derive(Debug, Deserialize)]
pub struct BaseResponse {
    base: String,
    currency: String,
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct CoinbasePriceResponse {
    pub data: BaseResponse,
}
pub type SpotPriceResponse = CoinbasePriceResponse;
pub type BuyPriceResponse = CoinbasePriceResponse;
pub type SellPriceResponse = CoinbasePriceResponse;

enum APIVersion {
    V2,
}

pub const API_BASE_URL: &str = "https://api.coinbase.com";

fn get_currency_string_for_url(currency: Currency) -> String {
    match currency {
        Currency::FiatCurrency(USD) => String::from("USD"),
        Currency::CryptoCurrency(BTC) => String::from("BTC")
    }
}


fn get_spot_price_url(version: APIVersion, currency: Currency) -> String {
    return format!(
        "{}/{}/prices/{}-{}/spot",
        API_BASE_URL,
        get_api_version_string(version),
        get_currency_string_for_url(Currency::CryptoCurrency(CryptoCurrency::BTC)),
        get_currency_string_for_url(currency)
    );
}

fn get_buy_price_url(version: APIVersion, currency: Currency) -> String {
    return format!(
        "{}/{}/prices/BTC-{}/buy",
        API_BASE_URL,
        get_api_version_string(version),
        get_currency_string_for_url(currency)
    );
}

fn get_sell_price_url(version: APIVersion, currency: Currency) -> String {
    return format!(
        "{}/{}/prices/BTC-{}/sell",
        API_BASE_URL,
        get_api_version_string(version),
        get_currency_string_for_url(currency)
    );
}

fn get_api_version_string(version: APIVersion) -> String {
    match version {
        APIVersion::V2 => String::from("v2")
    }
}

pub fn request_spot_price() -> SpotPriceResponse {
    let version: APIVersion = APIVersion::V2;
    let currency: Currency = Currency::FiatCurrency(USD);
    let request_url: String = get_spot_price_url(version, currency);
    let response: CoinbasePriceResponse = request::request(request_url);
    return response;
}

pub fn request_buy_price() -> BuyPriceResponse {
    let version: APIVersion = APIVersion::V2;
    let currency: Currency = Currency::FiatCurrency(USD);
    let request_url: String = get_buy_price_url(version, currency);
    let response: CoinbasePriceResponse = request::request(request_url);
    return response;
}

pub fn request_sell_price() -> SellPriceResponse {
    let version: APIVersion = APIVersion::V2;
    let currency: Currency = Currency::FiatCurrency(USD);
    let request_url: String = get_sell_price_url(version, currency);
    let response: CoinbasePriceResponse = request::request(request_url);
    return response;
}


