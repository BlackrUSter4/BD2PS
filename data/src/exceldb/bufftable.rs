// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bufftable {
    #[serde(rename = "AddBuffId")]
    pub add_buff_id: Option<i32>,
    #[serde(rename = "buffApplyType")]
    pub buff_apply_type: Option<i32>,
    #[serde(rename = "buffConditionId")]
    pub buff_condition_id: Option<i32>,
    #[serde(rename = "buffCountType")]
    pub buff_count_type: Option<i32>,
    #[serde(rename = "buffDescSkillTextId")]
    pub buff_desc_skill_text_id: Option<i32>,
    #[serde(rename = "buffDisplayType")]
    pub buff_display_type: Option<i32>,
    #[serde(rename = "buffDisplayValue")]
    pub buff_display_value: Option<i32>,
    #[serde(rename = "buffGroup", default)]
    pub buff_group: Vec<i32>,
    #[serde(rename = "buffIconSpriteName")]
    pub buff_icon_sprite_name: Option<String>,
    #[serde(rename = "buffMakerType")]
    pub buff_maker_type: Option<i32>,
    #[serde(rename = "buffSkillTextId")]
    pub buff_skill_text_id: Option<i32>,
    #[serde(rename = "buffTurn")]
    pub buff_turn: Option<i32>,
    #[serde(rename = "buffType")]
    pub buff_type: Option<i32>,
    #[serde(rename = "buffValue")]
    pub buff_value: Option<f32>,
    #[serde(rename = "classType")]
    pub class_type: String,
    #[serde(rename = "conditionAddBuffId")]
    pub condition_add_buff_id: Option<i32>,
    #[serde(rename = "effectPrefabName")]
    pub effect_prefab_name: Option<String>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "magicValue")]
    pub magic_value: Option<f32>,
    #[serde(rename = "overLapCount")]
    pub over_lap_count: Option<i32>,
    #[serde(rename = "overLapMax")]
    pub over_lap_max: Option<i32>,
    #[serde(rename = "ownerType")]
    pub owner_type: Option<i32>,
    #[serde(rename = "specialEffectPrefabName")]
    pub special_effect_prefab_name: Option<String>,
    #[serde(rename = "statType")]
    pub stat_type: Option<i32>,
    #[serde(rename = "subType")]
    pub sub_type: Option<i32>,
}

pub struct BufftableTable {
    records: Vec<Bufftable>,
    by_id: HashMap<i32, usize>,
}

impl BufftableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Bufftable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Bufftable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Bufftable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Bufftable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
