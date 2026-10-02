// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cinematalktable {
    #[serde(rename = "acceptTimeline", default)]
    pub accept_timeline: Option<i32>,
    #[serde(rename = "backgroundImgName", default)]
    pub background_img_name: Option<String>,
    #[serde(rename = "bubbleType", default)]
    pub bubble_type: Option<i32>,
    #[serde(rename = "dialogStoryTextId", default)]
    pub dialog_story_text_id: Option<i32>,
    #[serde(rename = "emotionType", default)]
    pub emotion_type: Option<i32>,
    #[serde(rename = "faceIllustName", default)]
    pub face_illust_name: Option<String>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "illustCharAnimation", default)]
    pub illust_char_animation: Option<Vec<String>>,
    #[serde(rename = "illustUniqueCharId", default)]
    pub illust_unique_char_id: Option<Vec<i32>>,
    #[serde(rename = "nameTextId", default)]
    pub name_text_id: Option<i32>,
    #[serde(rename = "packId", default)]
    pub pack_id: Option<i32>,
    #[serde(rename = "packIndex", default)]
    pub pack_index: Option<String>,
    #[serde(rename = "questGroupId", default)]
    pub quest_group_id: Option<i32>,
    #[serde(rename = "questGroupIndex", default)]
    pub quest_group_index: Option<i32>,
    #[serde(rename = "speakerName", default)]
    pub speaker_name: Option<String>,
    #[serde(rename = "state", default)]
    pub state: Option<i32>,
    #[serde(rename = "tabType", default)]
    pub tab_type: Option<i32>,
    #[serde(rename = "voiceResourceName", default)]
    pub voice_resource_name: Option<String>,
    #[serde(rename = "selectDialogId", default)]
    pub select_dialog_id: Option<i32>,
    #[serde(rename = "selectDialogTextIdIndex", default)]
    pub select_dialog_text_id_index: Option<i32>,
}

pub struct CinematalktableTable {
    records: Vec<Cinematalktable>,
    by_id: HashMap<i32, usize>,
    by_pack: HashMap<(i32, i32), usize>,
}

impl CinematalktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cinematalktable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_pack = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_pack.insert((record.pack_id.unwrap_or(1), record.id), idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_pack,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Cinematalktable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    /// Pack-scoped lookup — use this over `get()` for any new call site,
    /// since `id` collides across packs (see `pack_id` field doc).
    #[inline]
    pub fn get_by_pack(&self, pack_id: i32, id: i32) -> Option<&Cinematalktable> {
        self.by_pack.get(&(pack_id, id)).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Cinematalktable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Cinematalktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
