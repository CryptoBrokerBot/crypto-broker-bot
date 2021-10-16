use serenity::prelude::TypeMapKey;
use serde::{Serialize, Deserialize};

pub struct ApiKeyTypeMapKey;

impl TypeMapKey for ApiKeyTypeMapKey {
    type Value = String;
}

pub struct ApiBaseUrlTypeMapKey;

impl TypeMapKey for ApiBaseUrlTypeMapKey {
    type Value = String;
}

pub struct ApiConnectionDetails {
    pub api_key : String,
    pub api_base_url : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    pub success : bool,
    pub error_msg : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyResponse { }

#[derive(Deserialize, Serialize, Debug)]
pub struct BalanceResponse {
    pub user_id : String,
    pub balance : f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoPricePoint {
    #[serde(rename="asOf")]
    pub as_of : String,
    pub symbol : String,
    pub name : String,
    pub price : f64,
    #[serde(rename="imageUrl")]
    pub image_url : String,
    #[serde(rename="marketCap")]
    pub market_cap : f64,
    pub volume : f64,
    #[serde(rename="coingeckoTimestamp")]
    pub coingecko_timestamp : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyRewardRequest {
    pub user_id : String
}

pub type TimeseriesResponse = Vec<CryptoPricePoint>;
