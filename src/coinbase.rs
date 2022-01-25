use reqwest;
use serde::Deserialize;

const API_BASE_URL: &str = "https://api.coinbase.com";

#[derive(Debug, Deserialize)]
struct BaseResponse {
    base: String,
    currency: String,
    amount: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    data: BaseResponse,
}

enum APIVersion {
    V2,
}

fn get_price_from_response(response: Response) -> f32 {
    // Expected Response:
    //  {
    //    "data": {
    //      "base":"BTC",
    //      "currency":"USD",
    //      "amount":"39865.46"
    //    }
//    }
    let price: f32 = response.data.amount.parse().unwrap();
    return price;
}


fn get_api_version_string(version: APIVersion) -> String {
    match version {
        APIVersion::V2 => String::from("v2")
    }
}

fn get_spot_price_url() -> String {
    let version: APIVersion = APIVersion::V2;
    return format!("{}/{}/prices/spot", API_BASE_URL, get_api_version_string(version));
}


pub fn get_spot_price() -> f32 {
    let request_url = get_spot_price_url();
    let response = reqwest::blocking::get(request_url).unwrap();
    let response_json: Response = response.json().unwrap();
    let price = get_price_from_response(response_json);
    return price;
}
