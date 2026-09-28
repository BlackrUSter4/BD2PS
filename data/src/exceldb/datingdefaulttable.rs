// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datingdefaulttable {
    #[serde(rename = "datingApMax")]
    pub dating_ap_max: i32,
    #[serde(rename = "DailyPurchaseLimit")]
    pub daily_purchase_limit: Option<i32>,
    #[serde(rename = "PurchasePriceCount")]
    pub purchase_price_count: Option<Vec<i32>>,
    #[serde(rename = "PurchasePriceId")]
    pub purchase_price_id: Option<Vec<i32>>,
    #[serde(rename = "PurchasePriceType")]
    pub purchase_price_type: Option<Vec<i32>>,
}

pub struct DatingdefaulttableTable {
    records: Vec<Datingdefaulttable>,
}

impl DatingdefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Datingdefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Datingdefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Datingdefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
