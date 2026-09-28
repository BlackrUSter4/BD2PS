// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapuiobjecttable {
    #[serde(rename = "autoMoveInsideNpcId")]
    pub auto_move_inside_npc_id: Option<i32>,
    #[serde(rename = "autoMoveLocalTextId")]
    pub auto_move_local_text_id: Option<i32>,
    #[serde(rename = "availableAutoMove")]
    pub available_auto_move: Option<i32>,
    #[serde(rename = "iconSpriteName")]
    pub icon_sprite_name: Option<String>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapLocalTextId")]
    pub map_local_text_id: Option<i32>,
    #[serde(rename = "quikAutoMove")]
    pub quik_auto_move: Option<i32>,
    #[serde(rename = "quikAutoMoveIconName")]
    pub quik_auto_move_icon_name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
}

pub struct MapuiobjecttableTable {
    records: Vec<Mapuiobjecttable>,
    by_id: HashMap<i32, usize>,
}

impl MapuiobjecttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Mapuiobjecttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Mapuiobjecttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Mapuiobjecttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Mapuiobjecttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
