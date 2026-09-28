// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Talentskilltable {
    #[serde(rename = "catalystValue")]
    pub catalyst_value: Option<i32>,
    #[serde(rename = "classType")]
    pub class_type: i32,
    #[serde(rename = "costumeDesignId")]
    pub costume_design_id: Option<Vec<i32>>,
    #[serde(rename = "costumeId")]
    pub costume_id: Option<Vec<i32>>,
    #[serde(rename = "getExp")]
    pub get_exp: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "resetType")]
    pub reset_type: Option<i32>,
    #[serde(rename = "talentSkillDescLocalTextId")]
    pub talent_skill_desc_local_text_id: Option<Vec<i32>>,
    #[serde(rename = "talentSkillIconSpriteName")]
    pub talent_skill_icon_sprite_name: String,
    #[serde(rename = "talentSkillNameLocalTextId")]
    pub talent_skill_name_local_text_id: Option<Vec<i32>>,
    #[serde(rename = "targetType")]
    pub target_type: Option<i32>,
    #[serde(rename = "valueList")]
    pub value_list: Vec<f32>,
}

pub struct TalentskilltableTable {
    records: Vec<Talentskilltable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl TalentskilltableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Talentskilltable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Talentskilltable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Talentskilltable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Talentskilltable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Talentskilltable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
