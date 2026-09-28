// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigameskilltable {
    #[serde(rename = "attackCount")]
    pub attack_count: Option<i32>,
    #[serde(rename = "attackRange")]
    pub attack_range: Option<f32>,
    #[serde(rename = "attackSize")]
    pub attack_size: Option<f32>,
    #[serde(rename = "cooldown")]
    pub cooldown: Option<f32>,
    #[serde(rename = "duration")]
    pub duration: Option<f32>,
    #[serde(rename = "effectPrefabName")]
    pub effect_prefab_name: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "knockbackType")]
    pub knockback_type: Option<i32>,
    #[serde(rename = "pierce")]
    pub pierce: Option<i32>,
    #[serde(rename = "projectileSpeed")]
    pub projectile_speed: Option<f32>,
    #[serde(rename = "projectileTerm")]
    pub projectile_term: Option<f32>,
    #[serde(rename = "secondEffectPrefabName")]
    pub second_effect_prefab_name: Option<String>,
    #[serde(rename = "skillDescLocalTextId")]
    pub skill_desc_local_text_id: i32,
    #[serde(rename = "skillIconSpriteName")]
    pub skill_icon_sprite_name: String,
    #[serde(rename = "skillNameLocalTextId")]
    pub skill_name_local_text_id: i32,
    #[serde(rename = "skillValue")]
    pub skill_value: f32,
    #[serde(rename = "statType")]
    pub stat_type: Option<i32>,
    #[serde(rename = "upgradeDescLocalTextId")]
    pub upgrade_desc_local_text_id: i32,
}

pub struct FieldminigameskilltableTable {
    records: Vec<Fieldminigameskilltable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl FieldminigameskilltableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldminigameskilltable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldminigameskilltable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Fieldminigameskilltable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldminigameskilltable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldminigameskilltable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
