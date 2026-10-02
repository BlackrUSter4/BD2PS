// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigamesurvivaltable {
    #[serde(rename = "bombItemPrefab", default)]
    pub bomb_item_prefab: String,
    #[serde(rename = "bossMonsterSpawnRange", default)]
    pub boss_monster_spawn_range: f32,
    #[serde(rename = "bossRageTime", default)]
    pub boss_rage_time: i32,
    #[serde(rename = "bossRageValue", default)]
    pub boss_rage_value: i32,
    #[serde(rename = "bossSpawnPointPrefab", default)]
    pub boss_spawn_point_prefab: String,
    #[serde(rename = "boxNavPrefab", default)]
    pub box_nav_prefab: String,
    #[serde(rename = "characterGroupId", default)]
    pub character_group_id: i32,
    #[serde(rename = "eventMissionGroupId", default)]
    pub event_mission_group_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "itemLimitCount", default)]
    pub item_limit_count: i32,
    #[serde(rename = "knockbackRange", default)]
    pub knockback_range: f32,
    #[serde(rename = "knockbackTime", default)]
    pub knockback_time: f32,
    #[serde(rename = "levelupProtectionTime", default)]
    pub levelup_protection_time: f32,
    #[serde(rename = "loadingPrefabName", default)]
    pub loading_prefab_name: String,
    #[serde(rename = "mapGroupId", default)]
    pub map_group_id: i32,
    #[serde(rename = "monsterAttackInterval", default)]
    pub monster_attack_interval: f32,
    #[serde(rename = "monsterSpawnRange", default)]
    pub monster_spawn_range: f32,
    #[serde(rename = "monsterSpwanSpotCount", default)]
    pub monster_spwan_spot_count: i32,
    #[serde(rename = "normalPickupSpeed", default)]
    pub normal_pickup_speed: f32,
    #[serde(rename = "playerAttackInterval", default)]
    pub player_attack_interval: f32,
    #[serde(rename = "skillCapacity", default)]
    pub skill_capacity: i32,
    #[serde(rename = "spawnLimitCount", default)]
    pub spawn_limit_count: i32,
    #[serde(rename = "stunEffectPrefabName", default)]
    pub stun_effect_prefab_name: String,
    #[serde(rename = "timeLimit", default)]
    pub time_limit: i32,
    #[serde(rename = "ultimateReadyPrefab", default)]
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
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldminigamesurvivaltable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
