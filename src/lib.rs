use reqwest;
use serde::Deserialize;

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


fn get_price_from_response(response: Response) -> f32 {
  // expected response:
  //  {"data":{"base":"BTC","currency":"USD","amount":"39865.46"}}
  let price: f32 = response.data.amount.parse().unwrap();
  return price
}

pub fn get_coinbase_price() -> f32 {
            let response = reqwest::blocking::get("https://api.coinbase.com/v2/prices/spot").unwrap();
            let response_json: Response = response.json().unwrap();
            let price = get_price_from_response(response_json);
	    return price;
}
