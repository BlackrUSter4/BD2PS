// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Costumedesigntable {
    #[serde(rename = "costumeId")]
    pub costume_id: Option<i32>,
    #[serde(rename = "faceIconName")]
    pub face_icon_name: Option<String>,
    #[serde(rename = "faceIllustName")]
    pub face_illust_name: Option<String>,
    #[serde(rename = "iconSpriteName")]
    pub icon_sprite_name: Option<String>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "illustName")]
    pub illust_name: Option<String>,
    #[serde(rename = "inventoryIllustName")]
    pub inventory_illust_name: Option<String>,
    #[serde(rename = "isCollabo")]
    pub is_collabo: Option<i32>,
    #[serde(rename = "lobbyCutscene")]
    pub lobby_cutscene: Option<String>,
    #[serde(rename = "loopPrefabName")]
    pub loop_prefab_name: Option<Vec<String>>,
    #[serde(rename = "prefabName")]
    pub prefab_name: Option<String>,
    #[serde(rename = "simpleIllustName")]
    pub simple_illust_name: Option<String>,
    #[serde(rename = "skillIllustName")]
    pub skill_illust_name: Option<Vec<String>>,
    #[serde(rename = "skillTimelineName")]
    pub skill_timeline_name: Option<String>,
    #[serde(rename = "voiceResourceName")]
    pub voice_resource_name: Option<String>,
    #[serde(rename = "burstCutInNameTextId")]
    pub burst_cut_in_name_text_id: Option<Vec<i32>>,
}

pub struct CostumedesigntableTable {
    records: Vec<Costumedesigntable>,
    by_id: HashMap<i32, usize>,
}

impl CostumedesigntableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Costumedesigntable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Costumedesigntable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Costumedesigntable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Costumedesigntable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
