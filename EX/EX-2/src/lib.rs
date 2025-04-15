#![allow(non_snake_case)] //EX-2ではsnake_caseを使わない

use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Deserialize, Debug, Serialize)]
pub struct ApiResponse {
    pub message: String,
}

pub async fn fetch_data(base_url: &str) -> Result<ApiResponse, reqwest::Error> {
    let url = format!("{}/hello", base_url);
    let client = Client::new();
    let res = client.get(url).send().await?.json::<ApiResponse>().await?;

    Ok(res)
}