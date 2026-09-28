// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rlgrowthtable {
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "growthDescLocalTextId")]
    pub growth_desc_local_text_id: i32,
    #[serde(rename = "growthIcon")]
    pub growth_icon: String,
    #[serde(rename = "growthNameLocalTextId")]
    pub growth_name_local_text_id: i32,
    #[serde(rename = "growthType")]
    pub growth_type: i32,
    #[serde(rename = "growthValue")]
    pub growth_value: f32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "priceCount")]
    pub price_count: i32,
    #[serde(rename = "priceType")]
    pub price_type: i32,
}

pub struct RlgrowthtableTable {
    records: Vec<Rlgrowthtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl RlgrowthtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rlgrowthtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Rlgrowthtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Rlgrowthtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rlgrowthtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Rlgrowthtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
