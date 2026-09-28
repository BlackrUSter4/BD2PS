// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievementtable {
    #[serde(rename = "conditionSubType")]
    pub condition_sub_type: Option<i32>,
    #[serde(rename = "conditionType")]
    pub condition_type: i32,
    #[serde(rename = "conditionValue")]
    pub condition_value: f32,
    #[serde(rename = "contentsGroup")]
    pub contents_group: Option<i32>,
    #[serde(rename = "descLocalTextId")]
    pub desc_local_text_id: i32,
    #[serde(rename = "eventType")]
    pub event_type: i32,
    #[serde(rename = "exp")]
    pub exp: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "iconName")]
    pub icon_name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<i32>,
    #[serde(rename = "rewardItemCount")]
    pub reward_item_count: Option<Vec<i32>>,
    #[serde(rename = "rewardItemId")]
    pub reward_item_id: Option<Vec<i32>>,
    #[serde(rename = "rewardItemType")]
    pub reward_item_type: Option<Vec<i32>>,
    #[serde(rename = "shortCutId")]
    pub short_cut_id: Option<i32>,
    #[serde(rename = "tabType")]
    pub tab_type: Option<i32>,
    #[serde(rename = "titleLocalTextId")]
    pub title_local_text_id: i32,
    #[serde(rename = "useBlind")]
    pub use_blind: Option<i32>,
    #[serde(rename = "useType")]
    pub use_type: Option<i32>,
    #[serde(rename = "overCount")]
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
