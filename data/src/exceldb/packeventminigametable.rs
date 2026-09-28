// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packeventminigametable {
    #[serde(rename = "contentGenreId")]
    pub content_genre_id: Option<i32>,
    #[serde(rename = "eventClearType")]
    pub event_clear_type: i32,
    #[serde(rename = "eventClearValue")]
    pub event_clear_value: Option<i32>,
    #[serde(rename = "eventDescLocalTextId")]
    pub event_desc_local_text_id: i32,
    #[serde(rename = "eventNameLocalTextId")]
    pub event_name_local_text_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: Option<i32>,
    #[serde(rename = "miniGameRewardGroupId")]
    pub mini_game_reward_group_id: Option<i32>,
    #[serde(rename = "minigameHubBanner")]
    pub minigame_hub_banner: Option<String>,
    #[serde(rename = "packId")]
    pub pack_id: Option<i32>,
    #[serde(rename = "pointPositionId")]
    pub point_position_id: Option<i32>,
    #[serde(rename = "prefabName")]
    pub prefab_name: Option<String>,
    #[serde(rename = "staticCostumeId")]
    pub static_costume_id: Option<i32>,
    #[serde(rename = "staticCostumePath")]
    pub static_costume_path: Option<String>,
    #[serde(rename = "staticCostumeType")]
    pub static_costume_type: Option<i32>,
}

pub struct PackeventminigametableTable {
    records: Vec<Packeventminigametable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl PackeventminigametableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Packeventminigametable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            if let Some(group_id) = record.mini_game_reward_group_id {
                by_group.entry(group_id).or_insert_with(Vec::new).push(idx);
            }
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Packeventminigametable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Packeventminigametable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Packeventminigametable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Packeventminigametable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
