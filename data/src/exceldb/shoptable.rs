// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shoptable {
    #[serde(rename = "bargainLocalTextId")]
    pub bargain_local_text_id: i32,
    #[serde(rename = "groupBadNoticeLocalTextId")]
    pub group_bad_notice_local_text_id: i32,
    #[serde(rename = "groupGoodMessageLocalTextId")]
    pub group_good_message_local_text_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapNameTextId")]
    pub map_name_text_id: i32,
    #[serde(rename = "npcLocalTextId")]
    pub npc_local_text_id: i32,
    #[serde(rename = "packId")]
    pub pack_id: i32,
    #[serde(rename = "priceId")]
    pub price_id: Vec<i32>,
    #[serde(rename = "priceType")]
    pub price_type: Vec<i32>,
    #[serde(rename = "resetCount")]
    pub reset_count: i32,
    #[serde(rename = "resetTermType")]
    pub reset_term_type: i32,
    #[serde(rename = "startDay")]
    pub start_day: i32,
}

pub struct ShoptableTable {
    records: Vec<Shoptable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl ShoptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Shoptable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.group_bad_notice_local_text_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Shoptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Shoptable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Shoptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Shoptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
