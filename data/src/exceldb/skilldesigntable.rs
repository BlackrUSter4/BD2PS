// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skilldesigntable {
    #[serde(rename = "cameraShakeType", default)]
    pub camera_shake_type: Option<i32>,
    #[serde(rename = "cameraZoomInType", default)]
    pub camera_zoom_in_type: Option<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "skillIconSpriteName", default)]
    pub skill_icon_sprite_name: String,
}

pub struct SkilldesigntableTable {
    records: Vec<Skilldesigntable>,
    by_id: HashMap<i32, usize>,
}

impl SkilldesigntableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Skilldesigntable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Skilldesigntable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Skilldesigntable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Skilldesigntable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
