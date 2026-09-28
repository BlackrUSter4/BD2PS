// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skilltable {
    #[serde(rename = "attackMoveType")]
    pub attack_move_type: Option<i32>,
    #[serde(rename = "attackRange")]
    pub attack_range: Option<i32>,
    #[serde(rename = "attackRangeCount")]
    pub attack_range_count: i32,
    #[serde(rename = "attackType")]
    pub attack_type: Option<i32>,
    #[serde(rename = "buffId")]
    pub buff_id: Option<Vec<i32>>,
    #[serde(rename = "cooldown")]
    pub cooldown: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: Option<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "mainTargetType")]
    pub main_target_type: Option<i32>,
    #[serde(rename = "mainTargetValue")]
    pub main_target_value: Option<i32>,
    #[serde(rename = "repeatCount")]
    pub repeat_count: i32,
    #[serde(rename = "skillDescSkillTextId")]
    pub skill_desc_skill_text_id: i32,
    #[serde(rename = "skillDesignId")]
    pub skill_design_id: Option<i32>,
    #[serde(rename = "skillNameSkillTextId")]
    pub skill_name_skill_text_id: i32,
    #[serde(rename = "skillUseRate")]
    pub skill_use_rate: f32,
    #[serde(rename = "spReqCount")]
    pub sp_req_count: Option<i32>,
    #[serde(rename = "targetType")]
    pub target_type: Option<i32>,
}

pub struct SkilltableTable {
    records: Vec<Skilltable>,
    by_id: HashMap<i32, usize>,
}

impl SkilltableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Skilltable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Skilltable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Skilltable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Skilltable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
