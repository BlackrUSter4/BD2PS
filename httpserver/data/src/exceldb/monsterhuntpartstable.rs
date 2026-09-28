// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monsterhuntpartstable {
    #[serde(rename = "bossBodyPosition")]
    pub boss_body_position: Option<i32>,
    #[serde(rename = "bossPartIcon1")]
    pub boss_part_icon1: String,
    #[serde(rename = "bossPartIcon2")]
    pub boss_part_icon2: Option<String>,
    #[serde(rename = "conditionId")]
    pub condition_id: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "parentCharId")]
    pub parent_char_id: i32,
    #[serde(rename = "partLocalTextId")]
    pub part_local_text_id: i32,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
    #[serde(rename = "useCostume")]
    pub use_costume: Option<i32>,
    #[serde(rename = "weakDmgValue")]
    pub weak_dmg_value: Option<f32>,
}

pub struct MonsterhuntpartstableTable {
    records: Vec<Monsterhuntpartstable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl MonsterhuntpartstableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Monsterhuntpartstable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Monsterhuntpartstable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Monsterhuntpartstable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Monsterhuntpartstable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Monsterhuntpartstable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
