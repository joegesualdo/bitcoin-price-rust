use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;

const API_BASE_URL: &str = "https://api.kraken.com";

type AskResponse = [String; 3];
type BidResponse = [String; 3];
type LastTradeCloseResponse = [String; 2];
type VolumeResponse = [String; 2];
type VolumeWeightedAveragePriceResponse = [String; 2];
type NumberOfTradesResponse = [u32; 2];
type LowResponse = [String; 2];
type HighResponse = [String; 2];
type OpeningPriceResponse = String;

#[derive(Debug, Deserialize)]
pub struct XXBTZUSDResponse {
    pub a: AskResponse,
    pub b: BidResponse,
    pub c: LastTradeCloseResponse,
    pub v: VolumeResponse,
    pub p: VolumeWeightedAveragePriceResponse,
    pub t: NumberOfTradesResponse,
    pub l: LowResponse,
    pub h: HighResponse,
    pub o: OpeningPriceResponse,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ResultResponse {
    pub XXBTZUSD: XXBTZUSDResponse,
}

#[derive(Debug, Deserialize)]
pub struct KrakenResponse {
    // error:
    pub result: ResultResponse,
}

fn get_spot_price_url() -> String {
    return format!("{}/0/public/Ticker?pair=XBTUSD", API_BASE_URL);
}

pub fn request_market_data() -> Result<KrakenResponse> {
    let request_url: String = get_spot_price_url();
    let response_json: KrakenResponse = request::request(request_url)?;
    return Ok(response_json);
}
