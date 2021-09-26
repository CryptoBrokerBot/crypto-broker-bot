use serenity::framework::standard::CommandResult;
use serde_json;

use crate::types;

pub async fn list(conn_details : &types::ApiConnectionDetails) -> CommandResult<Vec<types::CryptoPricePoint>> {
    let result = reqwest::Client::new()
        .get(conn_details.api_base_url.clone() + "/list")
        .header("X-CB-API-KEY", conn_details.api_key.clone())
        .send()
        .await?;
    let response_str = result.text().await?;
    let list : types::ListResponse = serde_json::from_str(&response_str)?;
    Ok(list)
}