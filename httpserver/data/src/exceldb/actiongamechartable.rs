// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actiongamechartable {
    #[serde(rename = "charDescNameTextId")]
    pub char_desc_name_text_id: i32,
    #[serde(rename = "charIcon")]
    pub char_icon: String,
    #[serde(rename = "charIllustName")]
    pub char_illust_name: Option<String>,
    #[serde(rename = "charNameImage")]
    pub char_name_image: String,
    #[serde(rename = "charNameTextId")]
    pub char_name_text_id: i32,
    #[serde(rename = "charPrefabPath")]
    pub char_prefab_path: String,
    #[serde(rename = "charSkillGroupId")]
    pub char_skill_group_id: i32,
    #[serde(rename = "charStatId")]
    pub char_stat_id: i32,
    #[serde(rename = "charStatImageName")]
    pub char_stat_image_name: Option<String>,
    #[serde(rename = "charThumName")]
    pub char_thum_name: String,
    #[serde(rename = "charTitleNameTextId")]
    pub char_title_name_text_id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "hitEffPrefab")]
    pub hit_eff_prefab: Option<String>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "monsterPartsGroupId")]
    pub monster_parts_group_id: Option<i32>,
    #[serde(rename = "monsterType")]
    pub monster_type: Option<i32>,
    #[serde(rename = "voiceResourceName")]
    pub voice_resource_name: Option<String>,
}

pub struct ActiongamechartableTable {
    records: Vec<Actiongamechartable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl ActiongamechartableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Actiongamechartable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.char_skill_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Actiongamechartable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Actiongamechartable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actiongamechartable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Actiongamechartable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
