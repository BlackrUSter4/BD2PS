// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Missiontable {
    #[serde(rename = "conditionSubType", default)]
    pub condition_sub_type: Option<i32>,
    #[serde(rename = "conditionSubTypeParams", default)]
    pub condition_sub_type_params: Option<Vec<i32>>,
    #[serde(rename = "conditionType", default)]
    pub condition_type: Option<i32>,
    #[serde(rename = "conditionValue", default)]
    pub condition_value: i32,
    #[serde(rename = "descLocalTextId", default)]
    pub desc_local_text_id: i32,
    #[serde(rename = "groupId", default)]
    pub group_id: i32,
    #[serde(rename = "groupType", default)]
    pub group_type: Option<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "isConditionSubTypeMoreCheck", default)]
    pub is_condition_sub_type_more_check: Option<i32>,
    #[serde(rename = "isHighlight", default)]
    pub is_highlight: Option<i32>,
    #[serde(rename = "passExp", default)]
    pub pass_exp: Option<i32>,
    #[serde(rename = "rewardCount", default)]
    pub reward_count: Option<i32>,
    #[serde(rename = "rewardId", default)]
    pub reward_id: Option<i32>,
    #[serde(rename = "rewardType", default)]
    pub reward_type: Option<i32>,
    #[serde(rename = "shortCutId", default)]
    pub short_cut_id: Option<i32>,
    #[serde(rename = "sortId", default)]
    pub sort_id: i32,
    #[serde(rename = "titleLocalTextId", default)]
    pub title_local_text_id: i32,
}

pub struct MissiontableTable {
    records: Vec<Missiontable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl MissiontableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Missiontable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Missiontable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Missiontable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Missiontable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Missiontable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
