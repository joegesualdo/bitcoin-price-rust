use reqwest;
use reqwest::blocking::Response;
use serde::de::DeserializeOwned;

// Explains the DeserializeOwned: https://serde.rs/lifetimes.html

pub fn request<T>(
    url: String
) -> T 
    where T: DeserializeOwned {
        let response: Response = 
            reqwest::blocking::get(url)
            .unwrap();
        let response_json: T = 
            response.json()
            .unwrap();
        return response_json;
}
