// Source: https://docs.kraken.com/rest/
use reqwest;
use reqwest::blocking::Response;
use serde::Deserialize;

const API_BASE_URL: &str = "https://api.kraken.com";


type PriceResponse = [String; 2];

#[derive(Debug, Deserialize)]
struct XXBTZUSDResponse {
    c: PriceResponse,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ResultResponse {
    XXBTZUSD: XXBTZUSDResponse,
}

#[derive(Debug, Deserialize)]
struct KrakenResponse {
    // error: 
    result: ResultResponse,
}

fn get_price_from_response(response: KrakenResponse) -> f32 {
    // Expected Response:
    //  {
    //    "data": {
    //      "base":"BTC",
    //      "currency":"USD",
    //      "amount":"39865.46"
    //    }
//    }
    let price: f32 = response.result.XXBTZUSD.c.first().unwrap().parse().unwrap();
    return price;
}

fn get_spot_price_url() -> String {
    return format!("{}/0/public/Ticker?pair=XBTUSD", API_BASE_URL);
}


pub fn get_spot_price() -> f32 {
    let request_url: String = get_spot_price_url();
    let response: Response = reqwest::blocking::get(request_url).unwrap();
    let response_json: KrakenResponse = response.json().unwrap();
    let price: f32 = get_price_from_response(response_json);
    return price;
}
