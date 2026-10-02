// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Totalwardefaulttable {
    #[serde(rename = "idleChangeDamage", default)]
    pub idle_change_damage: Vec<i32>,
    #[serde(rename = "rewardObjectId", default)]
    pub reward_object_id: i32,
    #[serde(rename = "rewardObjectInteractionLocalTextId", default)]
    pub reward_object_interaction_local_text_id: i32,
    #[serde(rename = "rewardObjectPrefabName", default)]
    pub reward_object_prefab_name: String,
    #[serde(rename = "totalWarPresetBaseCount", default)]
    pub total_war_preset_base_count: i32,
    #[serde(rename = "totalWarPresetBuyCount", default)]
    pub total_war_preset_buy_count: i32,
    #[serde(rename = "totalWarPresetBuyType", default)]
    pub total_war_preset_buy_type: i32,
    #[serde(rename = "totalWarPresetMaxCount", default)]
    pub total_war_preset_max_count: i32,
    #[serde(rename = "totalWarStartBuffId", default)]
    pub total_war_start_buff_id: i32,
}

pub struct TotalwardefaulttableTable {
    records: Vec<Totalwardefaulttable>,
}

impl TotalwardefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Totalwardefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Totalwardefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Totalwardefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
