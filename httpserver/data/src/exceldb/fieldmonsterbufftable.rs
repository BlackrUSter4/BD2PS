// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldmonsterbufftable {
    #[serde(rename = "alertDetectionAngle")]
    pub alert_detection_angle: f32,
    #[serde(rename = "alertDetectionDistance")]
    pub alert_detection_distance: f32,
    #[serde(rename = "buffTIme")]
    pub buff_t_ime: f32,
    #[serde(rename = "detectionIgnoreWall")]
    pub detection_ignore_wall: i32,
    #[serde(rename = "distanceChase")]
    pub distance_chase: f32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f32,
}

pub struct FieldmonsterbufftableTable {
    records: Vec<Fieldmonsterbufftable>,
    by_id: HashMap<i32, usize>,
}

impl FieldmonsterbufftableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldmonsterbufftable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldmonsterbufftable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldmonsterbufftable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Fieldmonsterbufftable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
