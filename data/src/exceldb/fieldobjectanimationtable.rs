// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldobjectanimationtable {
    #[serde(rename = "actionAnimationName")]
    pub action_animation_name: String,
    #[serde(rename = "disableTimeline")]
    pub disable_timeline: Option<Vec<String>>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "idleAnimationName")]
    pub idle_animation_name: String,
    #[serde(rename = "isEnableTimeline")]
    pub is_enable_timeline: i32,
    #[serde(rename = "questRange")]
    pub quest_range: Vec<i32>,
}

pub struct FieldobjectanimationtableTable {
    records: Vec<Fieldobjectanimationtable>,
    by_id: HashMap<i32, usize>,
}

impl FieldobjectanimationtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldobjectanimationtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldobjectanimationtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldobjectanimationtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldobjectanimationtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
