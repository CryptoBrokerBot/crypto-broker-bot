use serenity::framework::standard::CommandResult;
use serde_json;

use crate::types;

macro_rules! api {
    ($fname:ident;$method:ident;$url:literal;$($i:ident : $t:ty),+;$result:ty) => {
        pub async fn $fname($($i : $t,)* conn_details : &types::ApiConnectionDetails) -> CommandResult<$result> {
            let result = reqwest::Client::new()
                .$method(conn_details.api_base_url.clone() + $url)
                .header("X-CB-API-KEY", conn_details.api_key.clone())
                .query(&[$((stringify!($i), &$i.to_string())),*])
                .send()
                .await?;
            println!("Got status code: {}", &result.status());
            if (!result.status().is_success()) {
                Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{} failed got {}", stringify!($fname), result.status().as_u16())))?;
            }
            let response_str = result.text().await?;
            println!("Got response str: {}", response_str);
            let response : $result = serde_json::from_str(&response_str)?;
            Ok(response)
        }
    }
}

// Can't use api! since it doesn't have any query parameters
pub async fn list(conn_details : &types::ApiConnectionDetails) -> CommandResult<Vec<types::CryptoPricePoint>> {
    let result = reqwest::Client::new()
        .get(conn_details.api_base_url.clone() + "/list")
        .header("X-CB-API-KEY", conn_details.api_key.clone())
        .send()
        .await?;
    let response_str = result.text().await?;
    let list : types::TimeseriesResponse = serde_json::from_str(&response_str)?;
    Ok(list)
}

api!(daily_reward; post; "/daily-reward"; user_id : u64; types::StatusResponse);
api!(balance; get; "/balance"; user_id : u64; types::BalanceResponse);
api!(buy; post; "/buy"; user_id : u64, qty : u64, symbol : &str; types::EmptyResponse);
