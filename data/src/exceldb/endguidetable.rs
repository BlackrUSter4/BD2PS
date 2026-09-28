// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endguidetable {
    #[serde(rename = "atlasName")]
    pub atlas_name: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "guideDescLocalTextId")]
    pub guide_desc_local_text_id: i32,
    #[serde(rename = "guideNameLocalTextId")]
    pub guide_name_local_text_id: i32,
    #[serde(rename = "guideTextureName")]
    pub guide_texture_name: Option<String>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "spinePrefabName")]
    pub spine_prefab_name: String,
    #[serde(rename = "spriteName")]
    pub sprite_name: Option<String>,
}

pub struct EndguidetableTable {
    records: Vec<Endguidetable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl EndguidetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Endguidetable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Endguidetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Endguidetable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Endguidetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Endguidetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
