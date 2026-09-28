// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eventmissiongrouptable {
    #[serde(rename = "eventMissionType")]
    pub event_mission_type: Option<i32>,
    #[serde(rename = "eventNameTextId")]
    pub event_name_text_id: Option<i32>,
    #[serde(rename = "guideDescLocalTextId")]
    pub guide_desc_local_text_id: Option<Vec<i32>>,
    #[serde(rename = "guideTitleLocalTextId")]
    pub guide_title_local_text_id: Option<Vec<i32>>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isCompleteHide")]
    pub is_complete_hide: Option<i32>,
    #[serde(rename = "missionGroupId")]
    pub mission_group_id: Vec<i32>,
    #[serde(rename = "scheduleType")]
    pub schedule_type: Option<i32>,
    #[serde(rename = "usePass")]
    pub use_pass: Option<i32>,
}

pub struct EventmissiongrouptableTable {
    records: Vec<Eventmissiongrouptable>,
    by_id: HashMap<i32, usize>,
}

impl EventmissiongrouptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Eventmissiongrouptable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Eventmissiongrouptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Eventmissiongrouptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Eventmissiongrouptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
