// Source: https://docs.ftx.com/#get-orderbook
use serde::Deserialize;
use crate::request;

const API_BASE_URL: &str = "https://ftx.com";

#[derive(Debug, Deserialize)]
pub struct ResultResponse {
    pub id: f32,
    pub price: f32,
    pub size: f32,
    pub side: String, 
    pub liquidation: bool,
    pub time: String,
}

#[derive(Debug, Deserialize)]
pub struct FTXTradesResponse {
    // error:
    pub result: Vec<ResultResponse>,
}

fn get_trades_url() -> String {
    return format!("{}/api/markets/BTC/USD/trades", API_BASE_URL);
}

pub fn request_trades_data() -> FTXTradesResponse {
    let request_url: String = get_trades_url();
    let response_json: FTXTradesResponse = request::request(request_url);
    return response_json
}

