// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipmenttable {
    #[serde(rename = "grade", default)]
    pub grade: i32,
    #[serde(rename = "growthGroupId", default)]
    pub growth_group_id: i32,
    #[serde(rename = "iconSpriteName", default)]
    pub icon_sprite_name: String,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "isMonsterHunt", default)]
    pub is_monster_hunt: Option<i32>,
    #[serde(rename = "itemAcquireId", default)]
    pub item_acquire_id: Vec<i32>,
    #[serde(rename = "itemDescNameTextId", default)]
    pub item_desc_name_text_id: i32,
    #[serde(rename = "itemNameTextId", default)]
    pub item_name_text_id: i32,
    #[serde(rename = "itemSubDescLocalTextId", default)]
    pub item_sub_desc_local_text_id: i32,
    #[serde(rename = "mainOptionGroupId", default)]
    pub main_option_group_id: Vec<i32>,
    #[serde(rename = "maxLevel", default)]
    pub max_level: i32,
    #[serde(rename = "notTrash", default)]
    pub not_trash: i32,
    #[serde(rename = "optionRerollId", default)]
    pub option_reroll_id: i32,
    #[serde(rename = "privateUniqueCharId", default)]
    pub private_unique_char_id: Option<i32>,
    #[serde(rename = "privateUniqueOptionGroupId", default)]
    pub private_unique_option_group_id: Option<Vec<i32>>,
    #[serde(rename = "qualityType", default)]
    pub quality_type: Option<i32>,
    #[serde(rename = "rankGroupId", default)]
    pub rank_group_id: i32,
    #[serde(rename = "slotType", default)]
    pub slot_type: Option<i32>,
    #[serde(rename = "subOptionGroupId", default)]
    pub sub_option_group_id: Vec<i32>,
    #[serde(rename = "uniqueEquipId", default)]
    pub unique_equip_id: i32,
    #[serde(rename = "expect", default)]
    pub expect: Option<i32>,
}

pub struct EquipmenttableTable {
    records: Vec<Equipmenttable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl EquipmenttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Equipmenttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Equipmenttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Equipmenttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Equipmenttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Equipmenttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
