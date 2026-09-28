// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Talenttable {
    #[serde(rename = "changeOff")]
    pub change_off: Option<i32>,
    #[serde(rename = "classType")]
    pub class_type: i32,
    #[serde(rename = "growthGroupId")]
    pub growth_group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isSkip")]
    pub is_skip: Option<i32>,
    #[serde(rename = "maxLevel")]
    pub max_level: i32,
    #[serde(rename = "skillEffectTalk1")]
    pub skill_effect_talk1: Option<i32>,
    #[serde(rename = "skillEffectTalk2")]
    pub skill_effect_talk2: Option<i32>,
    #[serde(rename = "talentDescNameTextId")]
    pub talent_desc_name_text_id: i32,
    #[serde(rename = "talentEffect")]
    pub talent_effect: String,
    #[serde(rename = "talentMark")]
    pub talent_mark: String,
    #[serde(rename = "talentNameTextId")]
    pub talent_name_text_id: i32,
    #[serde(rename = "talentSkillGroupId")]
    pub talent_skill_group_id: i32,
    #[serde(rename = "banPackId")]
    pub ban_pack_id: Option<Vec<i32>>,
    #[serde(rename = "fixedButtonHidden")]
    pub fixed_button_hidden: Option<i32>,
}

pub struct TalenttableTable {
    records: Vec<Talenttable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl TalenttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Talenttable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.growth_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Talenttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Talenttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Talenttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Talenttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
