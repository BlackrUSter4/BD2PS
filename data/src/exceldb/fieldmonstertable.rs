// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldmonstertable {
    #[serde(rename = "animatorType")]
    pub animator_type: Option<i32>,
    #[serde(rename = "battleDeckId")]
    pub battle_deck_id: Option<Vec<i32>>,
    #[serde(rename = "bundleType")]
    pub bundle_type: Option<i32>,
    #[serde(rename = "directionType")]
    pub direction_type: Option<i32>,
    #[serde(rename = "fieldMonsterAIId")]
    pub field_monster_a_i_id: i32,
    #[serde(rename = "hp")]
    pub hp: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "idleEffect")]
    pub idle_effect: Option<String>,
    #[serde(rename = "isAutoInteraction")]
    pub is_auto_interaction: i32,
    #[serde(rename = "monsterNameTextId")]
    pub monster_name_text_id: i32,
    #[serde(rename = "monsterSize")]
    pub monster_size: Option<i32>,
    #[serde(rename = "questRange")]
    pub quest_range: Option<Vec<i32>>,
    #[serde(rename = "regenId")]
    pub regen_id: Option<i32>,
    #[serde(rename = "resourceName")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType")]
    pub resource_type: Option<i32>,
    #[serde(rename = "rewardCount")]
    pub reward_count: Option<i32>,
    #[serde(rename = "rewardId")]
    pub reward_id: Option<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: Option<i32>,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
    #[serde(rename = "useBattleSkip")]
    pub use_battle_skip: Option<i32>,
}

pub struct FieldmonstertableTable {
    records: Vec<Fieldmonstertable>,
    by_id: HashMap<i32, usize>,
}

impl FieldmonstertableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldmonstertable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Fieldmonstertable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldmonstertable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldmonstertable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
