// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skywayfieldtable {
    #[serde(rename = "apType")]
    pub ap_type: i32,
    #[serde(rename = "battlePower")]
    pub battle_power: i32,
    #[serde(rename = "bossAp")]
    pub boss_ap: i32,
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "descLocalTextId")]
    pub desc_local_text_id: i32,
    #[serde(rename = "difficulty")]
    pub difficulty: Option<i32>,
    #[serde(rename = "displayRewardCount")]
    pub display_reward_count: Vec<i32>,
    #[serde(rename = "displayRewardId")]
    pub display_reward_id: Vec<i32>,
    #[serde(rename = "displayRewardType")]
    pub display_reward_type: Vec<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "iconSpriteName")]
    pub icon_sprite_name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "monsterAp")]
    pub monster_ap: Vec<i32>,
    #[serde(rename = "monsterId")]
    pub monster_id: Vec<i32>,
    #[serde(rename = "nameLocalTextId")]
    pub name_local_text_id: i32,
    #[serde(rename = "pointPositionId")]
    pub point_position_id: i32,
    #[serde(rename = "positionGroup")]
    pub position_group: i32,
    #[serde(rename = "worldMapPinId")]
    pub world_map_pin_id: i32,
}

pub struct SkywayfieldtableTable {
    records: Vec<Skywayfieldtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl SkywayfieldtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Skywayfieldtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Skywayfieldtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Skywayfieldtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Skywayfieldtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Skywayfieldtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
