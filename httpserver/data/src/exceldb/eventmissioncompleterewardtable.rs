// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eventmissioncompleterewardtable {}


pub struct EventmissioncompleterewardtableTable {
    records: Vec<Eventmissioncompleterewardtable>,
}

impl EventmissioncompleterewardtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Eventmissioncompleterewardtable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Eventmissioncompleterewardtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Eventmissioncompleterewardtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
