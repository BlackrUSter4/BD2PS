// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldmonstermovetable {
    #[serde(rename = "detectionAngle")]
    pub detection_angle: f32,
    #[serde(rename = "detectionDistance")]
    pub detection_distance: f32,
    #[serde(rename = "detectionIgnoreWall")]
    pub detection_ignore_wall: Option<i32>,
    #[serde(rename = "distanceChase")]
    pub distance_chase: f32,
    #[serde(rename = "distanceDefault")]
    pub distance_default: f32,
    #[serde(rename = "distanceRun")]
    pub distance_run: Option<f32>,
    #[serde(rename = "encounterAngle")]
    pub encounter_angle: f32,
    #[serde(rename = "encounterDistance")]
    pub encounter_distance: f32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "lasttimeChase")]
    pub lasttime_chase: f32,
    #[serde(rename = "lv")]
    pub lv: i32,
    #[serde(rename = "moveIgnoreWall")]
    pub move_ignore_wall: Option<i32>,
    #[serde(rename = "moving")]
    pub moving: i32,
    #[serde(rename = "speedChase")]
    pub speed_chase: f32,
    #[serde(rename = "speedDefault")]
    pub speed_default: f32,
    #[serde(rename = "speedRun")]
    pub speed_run: Option<f32>,
    #[serde(rename = "timeChase")]
    pub time_chase: f32,
    #[serde(rename = "timeDefault")]
    pub time_default: f32,
    #[serde(rename = "timeDiscovery")]
    pub time_discovery: Option<f32>,
    #[serde(rename = "timeRun")]
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
    pub fn iter(&self) -> std::slice::Iter<Fieldmonstermovetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
