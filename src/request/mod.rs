use anyhow::{Ok, Result};
use reqwest;
use reqwest::blocking::Response;
use serde::de::DeserializeOwned;

// Explains the DeserializeOwned: https://serde.rs/lifetimes.html

pub fn request<T>(url: String) -> Result<T>
where
    T: DeserializeOwned,
{
    let response: Response = reqwest::blocking::get(url)?;
    let response_json: T = response.json()?;
    return Ok(response_json);
}
