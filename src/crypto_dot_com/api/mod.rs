use crate::currencies::{CryptoCurrency, Currency, FiatCurrency};
use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;
use CryptoCurrency::*;
use FiatCurrency::*;

type URLString = String;

enum APIVersion {
    V2,
}

const API_BASE_URL: &str = "https://api.crypto.com";

pub fn get_currency_string_for_url(currency: Currency) -> URLString {
    match currency {
        Currency::FiatCurrency(USD) => String::from("USD"),
        Currency::CryptoCurrency(USDT) => String::from("USDT"),
        Currency::CryptoCurrency(BTC) => String::from("BTC"),
        _ => panic!("Currency not supported"),
    }
}

fn get_api_version_string(version: APIVersion) -> String {
    match version {
        APIVersion::V2 => String::from("v2"),
    }
}

type InstrumentNameResponse = String;
type CurrentBestBidResponse = f32;
type CurrentBestAskResponse = f32;
type PriceOfLatestTradeResponse = String;
type TimestampResponse = f32;
type TwentyFourHourTradedVolume = f32;
type PriceOfTwentyFourHourHighestTrade = f32;
type PriceOfTwentyFourHourLowestTrade = f32;
// type TwentyFourHourPriceChange = u32;

#[derive(Debug, Deserialize)]
pub struct DataResponse {
    // pub i: InstrumentNameResponse,
    // pub b: CurrentBestBidResponse,
    // pub k: CurrentBestAskResponse,
    pub a: PriceOfLatestTradeResponse,
    // pub t: TimestampResponse,
    // pub v: TwentyFourHourTradedVolume,
    // pub h: PriceOfTwentyFourHourHighestTrade,
    // pub l: PriceOfTwentyFourHourLowestTrade,
    // pub c: TwentyFourHourPriceChange,
}

#[derive(Debug, Deserialize)]
pub struct ResultResponse {
    // pub instrument_name: String,
    pub data: Vec<DataResponse>,
}
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Response {
    pub result: ResultResponse,
}

#[derive(Debug, Deserialize)]
pub struct CryptoDotComResponse {
    // error:
    pub result: ResultResponse,
}

pub fn get_ticker_url(currency: Currency) -> URLString {
    return format!(
        "{}/{}/public/get-ticker?instrument_name={}_{}",
        API_BASE_URL,
        get_api_version_string(APIVersion::V2),
        get_currency_string_for_url(Currency::CryptoCurrency(CryptoCurrency::BTC)),
        get_currency_string_for_url(currency)
    );
}

pub fn request_ticker_data() -> Result<CryptoDotComResponse> {
    let currency: Currency = Currency::CryptoCurrency(USDT);
    let request_url: String = get_ticker_url(currency);
    let response_json: CryptoDotComResponse = request::request(request_url)?;
    return Ok(response_json);
}
