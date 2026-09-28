// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigamemaptable {
    #[serde(rename = "clearGoalLocalTextId")]
    pub clear_goal_local_text_id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "monsterGroupId")]
    pub monster_group_id: i32,
    #[serde(rename = "monsterGrowthValue")]
    pub monster_growth_value: f32,
    #[serde(rename = "rewardCount")]
    pub reward_count: Option<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: Option<i32>,
    #[serde(rename = "stageClearValue")]
    pub stage_clear_value: i32,
    #[serde(rename = "stageDifficulty")]
    pub stage_difficulty: Option<i32>,
    #[serde(rename = "stageType")]
    pub stage_type: Option<i32>,
}

pub struct FieldminigamemaptableTable {
    records: Vec<Fieldminigamemaptable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl FieldminigamemaptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldminigamemaptable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldminigamemaptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Fieldminigamemaptable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldminigamemaptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldminigamemaptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
