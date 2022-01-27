// Source: https://developers.coinbase.com/docs/wallet/guides/price-data
use reqwest;
use reqwest::blocking::Response;
use serde::Deserialize;

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

fn get_spot_price_url(version: APIVersion) -> String {
    return format!(
        "{}/{}/prices/spot",
        API_BASE_URL,
        get_api_version_string(version)
    );
}


pub fn get_spot_price() -> f32 {
    let version: APIVersion = APIVersion::V2;
    let request_url: String = get_spot_price_url(version);
    let response: Response = 
        reqwest::blocking::get(request_url)
        .unwrap();
    let response_json: CoinbaseResponse = response.json().unwrap();
    let price: f32 = get_price_from_response(response_json);
    return price;
}
