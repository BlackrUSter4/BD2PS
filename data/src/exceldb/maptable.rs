// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maptable {
    #[serde(rename = "accelerationSpeed", default)]
    pub acceleration_speed: Option<f32>,
    #[serde(rename = "advantCharElement", default)]
    pub advant_char_element: Option<i32>,
    #[serde(rename = "ambienceName", default)]
    pub ambience_name: Option<String>,
    #[serde(rename = "audioCrossfade", default)]
    pub audio_crossfade: Option<i32>,
    #[serde(rename = "battleDeckId", default)]
    pub battle_deck_id: Option<Vec<i32>>,
    #[serde(rename = "commonSoundId", default)]
    pub common_sound_id: Option<i32>,
    #[serde(rename = "encounteSafeValue", default)]
    pub encounte_safe_value: Option<i32>,
    #[serde(rename = "encounterMaxValue", default)]
    pub encounter_max_value: Option<i32>,
    #[serde(rename = "envEffectResourceName", default)]
    pub env_effect_resource_name: Option<String>,
    #[serde(rename = "flashlightResouceName", default)]
    pub flashlight_resouce_name: Option<String>,
    #[serde(rename = "footStepSoundType", default)]
    pub foot_step_sound_type: Option<i32>,
    #[serde(rename = "gateId", default)]
    pub gate_id: Option<Vec<i32>>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "isInsideMap", default)]
    pub is_inside_map: Option<i32>,
    #[serde(rename = "mapEffect", default)]
    pub map_effect: Option<i32>,
    #[serde(rename = "mapEffectFreqMax", default)]
    pub map_effect_freq_max: Option<i32>,
    #[serde(rename = "mapEffectFreqMin", default)]
    pub map_effect_freq_min: Option<i32>,
    #[serde(rename = "mapGroupId", default)]
    pub map_group_id: i32,
    #[serde(rename = "mapNameTextId", default)]
    pub map_name_text_id: i32,
    #[serde(rename = "mapScale", default)]
    pub map_scale: Option<i32>,
    #[serde(rename = "mapScenePath", default)]
    pub map_scene_path: String,
    #[serde(rename = "mapType", default)]
    pub map_type: Option<i32>,
    #[serde(rename = "maxSpeed", default)]
    pub max_speed: Option<f32>,
    #[serde(rename = "minimapSize", default)]
    pub minimap_size: Option<String>,
    #[serde(rename = "minimapSpriteName", default)]
    pub minimap_sprite_name: Option<String>,
    #[serde(rename = "monsterElement", default)]
    pub monster_element: Option<i32>,
    #[serde(rename = "offset", default)]
    pub offset: Option<String>,
    #[serde(rename = "packId", default)]
    pub pack_id: i32,
    #[serde(rename = "showHpUi", default)]
    pub show_hp_ui: Option<i32>,
    #[serde(rename = "showQuestId", default)]
    pub show_quest_id: Option<i32>,
    #[serde(rename = "timeLineEffectUse", default)]
    pub time_line_effect_use: Option<i32>,
}

pub struct MaptableTable {
    records: Vec<Maptable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl MaptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Maptable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.map_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Maptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Maptable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Maptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Maptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
