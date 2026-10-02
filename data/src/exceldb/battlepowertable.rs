// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battlepowertable {
    #[serde(rename = "awakePower", default)]
    pub awake_power: i32,
    #[serde(rename = "costumeGradePower", default)]
    pub costume_grade_power: i32,
    #[serde(rename = "costumePower", default)]
    pub costume_power: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "imprintGradePower", default)]
    pub imprint_grade_power: i32,
    #[serde(rename = "levelValuePower", default)]
    pub level_value_power: i32,
    #[serde(rename = "potentialConnectionNodePower", default)]
    pub potential_connection_node_power: i32,
    #[serde(rename = "potentialPublicNodePower", default)]
    pub potential_public_node_power: i32,
    #[serde(rename = "potentialSkillNodePower", default)]
    pub potential_skill_node_power: i32,
}

pub struct BattlepowertableTable {
    records: Vec<Battlepowertable>,
    by_id: HashMap<i32, usize>,
}

impl BattlepowertableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Battlepowertable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Battlepowertable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Battlepowertable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Battlepowertable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
