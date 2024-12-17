// Source; https://exchange-docs.crypto.com/spot/index.html#public-get-ticker
#![allow(dead_code)]
use anyhow::{anyhow, Error, Ok, Result};
use thiserror::Error;

mod api;

type Price = f32;

pub fn get_price() -> Result<Price> {
    let response_json: api::CryptoDotComResponse = api::request_ticker_data()?;
    let first_option = response_json.result.data.first();
    let first_result = match first_option {
        Some(first) => Ok(first),
        None => Err(anyhow!(
            "response_json.result.data.first doesn't work. Looks like array is empty"
        )),
    };
    let first = first_result?;
    let price = first.a.parse()?;
    Ok(price)
}
