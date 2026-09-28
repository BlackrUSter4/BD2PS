// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cashpackagetable {
    #[serde(rename = "badgeResourceName")]
    pub badge_resource_name: Option<String>,
    #[serde(rename = "bannerFontLocalTextId")]
    pub banner_font_local_text_id: Option<i32>,
    #[serde(rename = "categoryResourceName")]
    pub category_resource_name: Option<String>,
    #[serde(rename = "contentsGroupId")]
    pub contents_group_id: Option<i32>,
    #[serde(rename = "contentsLocalTextId")]
    pub contents_local_text_id: Option<i32>,
    #[serde(rename = "contentsResourceName")]
    pub contents_resource_name: Option<String>,
    #[serde(rename = "contentsSortId")]
    pub contents_sort_id: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isActiveOnEnter")]
    pub is_active_on_enter: Option<i32>,
    #[serde(rename = "packageType")]
    pub package_type: Option<i32>,
    #[serde(rename = "paidShopGroupId")]
    pub paid_shop_group_id: Option<i32>,
    #[serde(rename = "paidShopId")]
    pub paid_shop_id: i32,
    #[serde(rename = "priority")]
    pub priority: Option<i32>,
    #[serde(rename = "resourceName")]
    pub resource_name: Option<String>,
    #[serde(rename = "saleGroup")]
    pub sale_group: Option<i32>,
    #[serde(rename = "shortCutId")]
    pub short_cut_id: Option<i32>,
}

pub struct CashpackagetableTable {
    records: Vec<Cashpackagetable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl CashpackagetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cashpackagetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Cashpackagetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Cashpackagetable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Cashpackagetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Cashpackagetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
