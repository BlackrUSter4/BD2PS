// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigamesurvivaltable {
    #[serde(rename = "bombItemPrefab")]
    pub bomb_item_prefab: String,
    #[serde(rename = "bossMonsterSpawnRange")]
    pub boss_monster_spawn_range: f32,
    #[serde(rename = "bossRageTime")]
    pub boss_rage_time: i32,
    #[serde(rename = "bossRageValue")]
    pub boss_rage_value: i32,
    #[serde(rename = "bossSpawnPointPrefab")]
    pub boss_spawn_point_prefab: String,
    #[serde(rename = "boxNavPrefab")]
    pub box_nav_prefab: String,
    #[serde(rename = "characterGroupId")]
    pub character_group_id: i32,
    #[serde(rename = "eventMissionGroupId")]
    pub event_mission_group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemLimitCount")]
    pub item_limit_count: i32,
    #[serde(rename = "knockbackRange")]
    pub knockback_range: f32,
    #[serde(rename = "knockbackTime")]
    pub knockback_time: f32,
    #[serde(rename = "levelupProtectionTime")]
    pub levelup_protection_time: f32,
    #[serde(rename = "loadingPrefabName")]
    pub loading_prefab_name: String,
    #[serde(rename = "mapGroupId")]
    pub map_group_id: i32,
    #[serde(rename = "monsterAttackInterval")]
    pub monster_attack_interval: f32,
    #[serde(rename = "monsterSpawnRange")]
    pub monster_spawn_range: f32,
    #[serde(rename = "monsterSpwanSpotCount")]
    pub monster_spwan_spot_count: i32,
    #[serde(rename = "normalPickupSpeed")]
    pub normal_pickup_speed: f32,
    #[serde(rename = "playerAttackInterval")]
    pub player_attack_interval: f32,
    #[serde(rename = "skillCapacity")]
    pub skill_capacity: i32,
    #[serde(rename = "spawnLimitCount")]
    pub spawn_limit_count: i32,
    #[serde(rename = "stunEffectPrefabName")]
    pub stun_effect_prefab_name: String,
    #[serde(rename = "timeLimit")]
    pub time_limit: i32,
    #[serde(rename = "ultimateReadyPrefab")]
    pub ultimate_ready_prefab: String,
}

pub struct FieldminigamesurvivaltableTable {
    records: Vec<Fieldminigamesurvivaltable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl FieldminigamesurvivaltableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldminigamesurvivaltable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.character_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Fieldminigamesurvivaltable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Fieldminigamesurvivaltable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldminigamesurvivaltable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Fieldminigamesurvivaltable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
