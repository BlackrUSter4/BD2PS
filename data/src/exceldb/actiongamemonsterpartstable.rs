// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actiongamemonsterpartstable {
    #[serde(rename = "banMonsterPatternId")]
    pub ban_monster_pattern_id: Option<Vec<i32>>,
    #[serde(rename = "baseModelName")]
    pub base_model_name: Option<String>,
    #[serde(rename = "blowWeakPartsValue")]
    pub blow_weak_parts_value: Option<f32>,
    #[serde(rename = "breakType")]
    pub break_type: Option<i32>,
    #[serde(rename = "brokenAnimationName")]
    pub broken_animation_name: Option<String>,
    #[serde(rename = "brokenModelName")]
    pub broken_model_name: Option<String>,
    #[serde(rename = "brokenValue")]
    pub broken_value: Option<i32>,
    #[serde(rename = "cutWeakPartsValue")]
    pub cut_weak_parts_value: Option<f32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "partsNameTextId")]
    pub parts_name_text_id: Option<i32>,
}

pub struct ActiongamemonsterpartstableTable {
    records: Vec<Actiongamemonsterpartstable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl ActiongamemonsterpartstableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Actiongamemonsterpartstable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Actiongamemonsterpartstable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Actiongamemonsterpartstable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actiongamemonsterpartstable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actiongamemonsterpartstable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
