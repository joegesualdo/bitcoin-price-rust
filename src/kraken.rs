// Source: https://docs.kraken.com/rest/
use serde::Deserialize;

#[path = "./request.rs"]
mod request;

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
struct XXBTZUSDResponse {
    a: AskResponse,
    b: BidResponse,
    c: LastTradeCloseResponse,
    v: VolumeResponse,
    p: VolumeWeightedAveragePriceResponse,
    t: NumberOfTradesResponse,
    l: LowResponse,
    h: HighResponse,
    o: OpeningPriceResponse,
}

type Price = f32;
struct Ask {
    price: Price,
    whole_lot_volume: u32,
    lot_volume: f32,
}

struct Bid {
    price: Price,
    whole_lot_volume: u32,
    lot_volume: f32,
}

struct LastTradeClosed {
    price: Price,
    lot_volume: f32,
}

struct Volume {
    today: f32,
    last_twenty_four_hours: f32,
}

struct VolumeWeightedAveragePrice {
    today: Price,
    last_twenty_four_hours: Price,
}
struct NumberOfTrades {
    today: u32,
    last_twenty_four_hours: u32,
}
struct LowPrice {
    today: Price,
    last_twenty_four_hours: Price,
}
struct HighPrice {
    today: Price,
    last_twenty_four_hours: Price,
}

struct MarketData {
    ask: Ask,
    bid: Bid,
    last_trade_closed: LastTradeClosed,
    volume: Volume,
    volume_weighted_average_price: VolumeWeightedAveragePrice,
    number_of_trades: NumberOfTrades,
    low_price: LowPrice,
    high_price: HighPrice,
    todays_opening_price: Price,
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

fn convert_response_to_market_data(response: KrakenResponse) -> MarketData {
    MarketData {
        ask: Ask {
            price: response.result.XXBTZUSD.a[0].parse().unwrap(),
            whole_lot_volume: response.result.XXBTZUSD.a[1].parse().unwrap(),
            lot_volume: response.result.XXBTZUSD.a[2].parse().unwrap(),
        },
        bid: Bid {
            price: response.result.XXBTZUSD.b[0].parse().unwrap(),
            whole_lot_volume: response.result.XXBTZUSD.b[1].parse().unwrap(),
            lot_volume: response.result.XXBTZUSD.b[2].parse().unwrap(),
        },
        last_trade_closed: LastTradeClosed {
            price: response.result.XXBTZUSD.c[0].parse().unwrap(),
            lot_volume: response.result.XXBTZUSD.c[1].parse().unwrap(),
        },
        volume: Volume {
            today: response.result.XXBTZUSD.v[0].parse().unwrap(),
            last_twenty_four_hours: response.result.XXBTZUSD.v[1].parse().unwrap(),
        },
        volume_weighted_average_price: VolumeWeightedAveragePrice {
            today: response.result.XXBTZUSD.p[0].parse().unwrap(),
            last_twenty_four_hours: response.result.XXBTZUSD.p[1].parse().unwrap(),
        },
        number_of_trades: NumberOfTrades {
            today: response.result.XXBTZUSD.t[0],
            last_twenty_four_hours: response.result.XXBTZUSD.t[1],
        },
        low_price: LowPrice {
            today: response.result.XXBTZUSD.l[0].parse().unwrap(),
            last_twenty_four_hours: response.result.XXBTZUSD.l[1].parse().unwrap(),
        },
        high_price: HighPrice {
            today: response.result.XXBTZUSD.h[0].parse().unwrap(),
            last_twenty_four_hours: response.result.XXBTZUSD.h[1].parse().unwrap(),
        },
        todays_opening_price: response.result.XXBTZUSD.o.parse().unwrap(),
    }
}

pub fn get_spot_price() -> f32 {
    let request_url: String = get_spot_price_url();
    let response_json: KrakenResponse = request::request(request_url);
    let market_data: MarketData = convert_response_to_market_data(response_json);
    let price: f32 = market_data.last_trade_closed.price;
    return price;
}
