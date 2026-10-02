// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigameskillgrouptable {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "maxLevel", default)]
    pub max_level: i32,
    #[serde(rename = "skillNameTextId", default)]
    pub skill_name_text_id: i32,
    #[serde(rename = "skillSubType", default)]
    pub skill_sub_type: i32,
    #[serde(rename = "skillType", default)]
    pub skill_type: Option<i32>,
    #[serde(rename = "targetType", default)]
    pub target_type: Option<i32>,
    #[serde(rename = "uniqueCharGroupId", default)]
    pub unique_char_group_id: Option<i32>,
    #[serde(rename = "upgradeSynergyId", default)]
    pub upgrade_synergy_id: Option<i32>,
}

pub struct FieldminigameskillgrouptableTable {
    records: Vec<Fieldminigameskillgrouptable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl FieldminigameskillgrouptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldminigameskillgrouptable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            if let Some(group_id) = record.unique_char_group_id {
                by_group.entry(group_id).or_insert_with(Vec::new).push(idx);
            }
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Fieldminigameskillgrouptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Fieldminigameskillgrouptable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldminigameskillgrouptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldminigameskillgrouptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
