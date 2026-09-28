// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldactionobjecttable {
    #[serde(rename = "bundleType")]
    pub bundle_type: i32,
    #[serde(rename = "disableTimeline")]
    pub disable_timeline: Option<Vec<String>>,
    #[serde(rename = "fieldObjectGroupId")]
    pub field_object_group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isEnableTimeline")]
    pub is_enable_timeline: Option<i32>,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f32,
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    #[serde(rename = "resourceType")]
    pub resource_type: i32,
}

pub struct FieldactionobjecttableTable {
    records: Vec<Fieldactionobjecttable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl FieldactionobjecttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldactionobjecttable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.field_object_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Fieldactionobjecttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Fieldactionobjecttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldactionobjecttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Fieldactionobjecttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
