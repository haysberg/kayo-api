use reqwest::{Error, header::{ACCEPT, CONTENT_TYPE}};
use serde_json::json;
use tracing::info;

pub async fn get_leagues() ->  Result<serde_json::Value, Error> {
    let client = reqwest::Client::new();

    let response = client
    .get("https://esports-api.service.valorantesports.com/persisted/val/getLeagues?hl=en-US&sport=val")
    .header(CONTENT_TYPE, "application/json")
    .header(ACCEPT, "application/json")
    .header("x-api-key", "0TvQnueqKa5mxJntVWt0w4LpLfEkrV1Ta8rQBb9Z")
    .send()
    .await
    .unwrap();

    let json : serde_json::Value = response.json().await.unwrap();

    info!("{json}");


    Ok(json!(["place", "holder"]))
}

pub fn init_data() -> Result<serde_json::Value, Error> {
    Ok(json!(["place", "holder"]))
}