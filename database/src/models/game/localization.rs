use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Localization {
    pub value: Value,
    pub result_code: String,
    pub result_msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    pub skus: HashMap<String, Skus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skus {
    pub price: String,
    pub price_micros: String,
    pub formatted_price: FormattedPrice,
    pub currency: Currency,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    Usd,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FormattedPrice {
    #[serde(rename = "$0.99")]
    The099,
    #[serde(rename = "$14.99")]
    The1499,
    #[serde(rename = "$16.99")]
    The1699,
    #[serde(rename = "$1.79")]
    The179,
    #[serde(rename = "$20.99")]
    The2099,
    #[serde(rename = "$25.99")]
    The2599,
    #[serde(rename = "$32.99")]
    The3299,
    #[serde(rename = "$4.29")]
    The429,
    #[serde(rename = "$42.99")]
    The4299,
    #[serde(rename = "$67.99")]
    The6799,
    #[serde(rename = "$7.99")]
    The799,
    #[serde(rename = "$8.49")]
    The849,
    #[serde(rename = "$89.99")]
    The8999,
}
