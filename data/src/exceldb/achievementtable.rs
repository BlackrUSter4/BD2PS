// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievementtable {
    #[serde(rename = "conditionSubType", default)]
    pub condition_sub_type: Option<i32>,
    #[serde(rename = "conditionType", default)]
    pub condition_type: i32,
    #[serde(rename = "conditionValue", default)]
    pub condition_value: f32,
    #[serde(rename = "contentsGroup", default)]
    pub contents_group: Option<i32>,
    #[serde(rename = "descLocalTextId", default)]
    pub desc_local_text_id: i32,
    #[serde(rename = "eventType", default)]
    pub event_type: i32,
    #[serde(rename = "exp", default)]
    pub exp: Option<i32>,
    #[serde(rename = "groupId", default)]
    pub group_id: i32,
    #[serde(rename = "iconName", default)]
    pub icon_name: String,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "parentGroupId", default)]
    pub parent_group_id: Option<i32>,
    #[serde(rename = "rewardItemCount", default)]
    pub reward_item_count: Option<Vec<i32>>,
    #[serde(rename = "rewardItemId", default)]
    pub reward_item_id: Option<Vec<i32>>,
    #[serde(rename = "rewardItemType", default)]
    pub reward_item_type: Option<Vec<i32>>,
    #[serde(rename = "shortCutId", default)]
    pub short_cut_id: Option<i32>,
    #[serde(rename = "tabType", default)]
    pub tab_type: Option<i32>,
    #[serde(rename = "titleLocalTextId", default)]
    pub title_local_text_id: i32,
    #[serde(rename = "useBlind", default)]
    pub use_blind: Option<i32>,
    #[serde(rename = "useType", default)]
    pub use_type: Option<i32>,
    #[serde(rename = "overCount", default)]
    pub over_count: Option<i64>,
}

pub struct AchievementtableTable {
    records: Vec<Achievementtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl AchievementtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Achievementtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Achievementtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Achievementtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Achievementtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Achievementtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
