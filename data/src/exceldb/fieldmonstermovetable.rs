// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldmonstermovetable {
    #[serde(rename = "detectionAngle", default)]
    pub detection_angle: f32,
    #[serde(rename = "detectionDistance", default)]
    pub detection_distance: f32,
    #[serde(rename = "detectionIgnoreWall", default)]
    pub detection_ignore_wall: Option<i32>,
    #[serde(rename = "distanceChase", default)]
    pub distance_chase: f32,
    #[serde(rename = "distanceDefault", default)]
    pub distance_default: f32,
    #[serde(rename = "distanceRun", default)]
    pub distance_run: Option<f32>,
    #[serde(rename = "encounterAngle", default)]
    pub encounter_angle: f32,
    #[serde(rename = "encounterDistance", default)]
    pub encounter_distance: f32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "lasttimeChase", default)]
    pub lasttime_chase: f32,
    #[serde(rename = "lv", default)]
    pub lv: i32,
    #[serde(rename = "moveIgnoreWall", default)]
    pub move_ignore_wall: Option<i32>,
    #[serde(rename = "moving", default)]
    pub moving: i32,
    #[serde(rename = "speedChase", default)]
    pub speed_chase: f32,
    #[serde(rename = "speedDefault", default)]
    pub speed_default: f32,
    #[serde(rename = "speedRun", default)]
    pub speed_run: Option<f32>,
    #[serde(rename = "timeChase", default)]
    pub time_chase: f32,
    #[serde(rename = "timeDefault", default)]
    pub time_default: f32,
    #[serde(rename = "timeDiscovery", default)]
    pub time_discovery: Option<f32>,
    #[serde(rename = "timeRun", default)]
    pub time_run: Option<f32>,
}

pub struct FieldmonstermovetableTable {
    records: Vec<Fieldmonstermovetable>,
    by_id: HashMap<i32, usize>,
}

impl FieldmonstermovetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldmonstermovetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldmonstermovetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldmonstermovetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldmonstermovetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
