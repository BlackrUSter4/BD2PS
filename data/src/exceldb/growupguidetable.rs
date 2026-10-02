// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Growupguidetable {
    #[serde(rename = "category", default)]
    pub category: i32,
    #[serde(rename = "categoryId", default)]
    pub category_id: i32,
    #[serde(rename = "descText", default)]
    pub desc_text: i32,
    #[serde(rename = "groupId", default)]
    pub group_id: i32,
    #[serde(rename = "guideTutorialId", default)]
    pub guide_tutorial_id: Option<Vec<i32>>,
    #[serde(rename = "shortCutId", default)]
    pub short_cut_id: Option<i32>,
}

pub struct GrowupguidetableTable {
    records: Vec<Growupguidetable>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl GrowupguidetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Growupguidetable> = serde_json::from_str(&json)?;
        
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Growupguidetable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Growupguidetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Growupguidetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
