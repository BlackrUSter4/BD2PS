// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipoptionaltertable {
    #[serde(rename = "afterMain1OptGroupId", default)]
    pub after_main1_opt_group_id: Option<i32>,
    #[serde(rename = "afterMain1OptId", default)]
    pub after_main1_opt_id: Vec<i32>,
    #[serde(rename = "afterMain2OptGroupId", default)]
    pub after_main2_opt_group_id: Option<i32>,
    #[serde(rename = "afterMain2OptId", default)]
    pub after_main2_opt_id: Vec<i32>,
    #[serde(rename = "afterPrivateOptGroupId", default)]
    pub after_private_opt_group_id: Option<i32>,
    #[serde(rename = "afterPrivateOptId", default)]
    pub after_private_opt_id: Option<i32>,
    #[serde(rename = "beforeMain1OptGroupId", default)]
    pub before_main1_opt_group_id: Option<i32>,
    #[serde(rename = "beforeMain1OptId", default)]
    pub before_main1_opt_id: Vec<i32>,
    #[serde(rename = "beforeMain2OptGroupId", default)]
    pub before_main2_opt_group_id: Option<i32>,
    #[serde(rename = "beforeMain2OptId", default)]
    pub before_main2_opt_id: Vec<i32>,
    #[serde(rename = "beforePrivateOptGroupId", default)]
    pub before_private_opt_group_id: Option<i32>,
    #[serde(rename = "beforePrivateOptId", default)]
    pub before_private_opt_id: Option<i32>,
    #[serde(rename = "equipId", default)]
    pub equip_id: i32,
    #[serde(rename = "groupId", default)]
    pub group_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
}

pub struct EquipoptionaltertableTable {
    records: Vec<Equipoptionaltertable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl EquipoptionaltertableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Equipoptionaltertable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            if let Some(group_id) = record.after_private_opt_group_id {
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
    pub fn get(&self, id: i32) -> Option<&Equipoptionaltertable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Equipoptionaltertable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Equipoptionaltertable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Equipoptionaltertable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
