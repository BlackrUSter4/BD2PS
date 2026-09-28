// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alchemytable {
    #[serde(rename = "alchemyCategory")]
    pub alchemy_category: i32,
    #[serde(rename = "displayGroupId")]
    pub display_group_id: i32,
    #[serde(rename = "groupLocalTextId")]
    pub group_local_text_id: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "materialItemCount")]
    pub material_item_count: Vec<i32>,
    #[serde(rename = "materialItemId")]
    pub material_item_id: Vec<i32>,
    #[serde(rename = "materialItemType")]
    pub material_item_type: Vec<i32>,
    #[serde(rename = "resultItemCount")]
    pub result_item_count: i32,
    #[serde(rename = "resultItemId")]
    pub result_item_id: i32,
    #[serde(rename = "resultItemType")]
    pub result_item_type: i32,
    #[serde(rename = "talentLevel")]
    pub talent_level: i32,
}

pub struct AlchemytableTable {
    records: Vec<Alchemytable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl AlchemytableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Alchemytable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.display_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Alchemytable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Alchemytable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Alchemytable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Alchemytable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
