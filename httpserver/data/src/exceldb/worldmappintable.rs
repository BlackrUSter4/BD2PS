// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worldmappintable {
    #[serde(rename = "huntPackId")]
    pub hunt_pack_id: Option<i32>,
    #[serde(rename = "huntPinIcoSpriteName")]
    pub hunt_pin_ico_sprite_name: String,
    #[serde(rename = "huntTypeGroupId")]
    pub hunt_type_group_id: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapDescNameTextId")]
    pub map_desc_name_text_id: i32,
    #[serde(rename = "mapNameTextId")]
    pub map_name_text_id: i32,
    #[serde(rename = "mapPinIconSpriteName")]
    pub map_pin_icon_sprite_name: String,
    #[serde(rename = "mapPinThumbSpriteName")]
    pub map_pin_thumb_sprite_name: String,
    #[serde(rename = "packId")]
    pub pack_id: Option<i32>,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
}

pub struct WorldmappintableTable {
    records: Vec<Worldmappintable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl WorldmappintableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Worldmappintable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            if let Some(group_id) = record.hunt_type_group_id {
                by_group.entry(group_id).or_insert_with(Vec::new).push(idx);
            }
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Worldmappintable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Worldmappintable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Worldmappintable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Worldmappintable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
