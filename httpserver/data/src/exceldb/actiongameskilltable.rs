// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actiongameskilltable {
    #[serde(rename = "attackId")]
    pub attack_id: Option<i32>,
    #[serde(rename = "buffId")]
    pub buff_id: Option<Vec<i32>>,
    #[serde(rename = "comboType")]
    pub combo_type: Option<i32>,
    #[serde(rename = "comboValue")]
    pub combo_value: Option<i32>,
    #[serde(rename = "cooldown")]
    pub cooldown: Option<f32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "hitSoundName")]
    pub hit_sound_name: Option<String>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "skillAnimationName")]
    pub skill_animation_name: Option<String>,
    #[serde(rename = "skillCancelType")]
    pub skill_cancel_type: Option<i32>,
    #[serde(rename = "skillDescNameTextId")]
    pub skill_desc_name_text_id: Option<i32>,
    #[serde(rename = "skillIconSpriteName")]
    pub skill_icon_sprite_name: Option<String>,
    #[serde(rename = "skillNameTextId")]
    pub skill_name_text_id: Option<i32>,
    #[serde(rename = "skillType")]
    pub skill_type: i32,
    #[serde(rename = "skillTypeValue")]
    pub skill_type_value: Vec<f32>,
    #[serde(rename = "staminaValue")]
    pub stamina_value: Option<i32>,
}

pub struct ActiongameskilltableTable {
    records: Vec<Actiongameskilltable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl ActiongameskilltableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Actiongameskilltable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Actiongameskilltable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Actiongameskilltable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actiongameskilltable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Actiongameskilltable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
