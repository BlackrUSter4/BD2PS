// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cashproducttable {
    #[serde(rename = "appleInAppId")]
    pub apple_in_app_id: Option<String>,
    #[serde(rename = "googleInAppId")]
    pub google_in_app_id: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "priceCount")]
    pub price_count: Option<i32>,
    #[serde(rename = "priceId")]
    pub price_id: Option<i32>,
    #[serde(rename = "priceType")]
    pub price_type: Option<i32>,
    #[serde(rename = "priority")]
    pub priority: Option<i32>,
    #[serde(rename = "productLocalTextId")]
    pub product_local_text_id: i32,
    #[serde(rename = "purchaseLimitCount")]
    pub purchase_limit_count: Option<i32>,
    #[serde(rename = "purchaseLimitType")]
    pub purchase_limit_type: Option<i32>,
    #[serde(rename = "randomboxId")]
    pub randombox_id: i32,
    #[serde(rename = "saleGroup")]
    pub sale_group: Option<i32>,
    #[serde(rename = "timeLimitType")]
    pub time_limit_type: Option<i32>,
    #[serde(rename = "bulkOrderAvailability")]
    pub bulk_order_availability: Option<i32>,
}

pub struct CashproducttableTable {
    records: Vec<Cashproducttable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl CashproducttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cashproducttable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Cashproducttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Cashproducttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Cashproducttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Cashproducttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
