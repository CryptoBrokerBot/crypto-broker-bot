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

pub type ListResponse = Vec<CryptoPricePoint>;