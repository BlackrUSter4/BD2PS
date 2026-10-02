// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supportcharacterdefaulttable {
    #[serde(rename = "DailyMaxSupportRewardCount", default)]
    pub daily_max_support_reward_count: i32,
    #[serde(rename = "DailyRecommendedSupportCount", default)]
    pub daily_recommended_support_count: i32,
    #[serde(rename = "DailySupportCount", default)]
    pub daily_support_count: i32,
    #[serde(rename = "SupportCombatPower", default)]
    pub support_combat_power: i32,
    #[serde(rename = "SupportGuideTutorial", default)]
    pub support_guide_tutorial: i32,
    #[serde(rename = "SupportRewardItemType", default)]
    pub support_reward_item_type: i32,
    #[serde(rename = "SupportRewardValue", default)]
    pub support_reward_value: i32,
}

pub struct SupportcharacterdefaulttableTable {
    records: Vec<Supportcharacterdefaulttable>,
}

impl SupportcharacterdefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Supportcharacterdefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Supportcharacterdefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Supportcharacterdefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
