// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cinematalktable {
    #[serde(rename = "acceptTimeline")]
    pub accept_timeline: Option<i32>,
    #[serde(rename = "backgroundImgName")]
    pub background_img_name: Option<String>,
    #[serde(rename = "bubbleType")]
    pub bubble_type: Option<i32>,
    #[serde(rename = "dialogStoryTextId")]
    pub dialog_story_text_id: Option<i32>,
    #[serde(rename = "emotionType")]
    pub emotion_type: Option<i32>,
    #[serde(rename = "faceIllustName")]
    pub face_illust_name: Option<String>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "illustCharAnimation")]
    pub illust_char_animation: Option<Vec<String>>,
    #[serde(rename = "illustUniqueCharId")]
    pub illust_unique_char_id: Option<Vec<i32>>,
    #[serde(rename = "nameTextId")]
    pub name_text_id: Option<i32>,
    #[serde(rename = "packId")]
    pub pack_id: Option<i32>,
    #[serde(rename = "packIndex")]
    pub pack_index: Option<String>,
    #[serde(rename = "questGroupId")]
    pub quest_group_id: Option<i32>,
    #[serde(rename = "questGroupIndex")]
    pub quest_group_index: Option<i32>,
    #[serde(rename = "speakerName")]
    pub speaker_name: Option<String>,
    #[serde(rename = "state")]
    pub state: Option<i32>,
    #[serde(rename = "tabType")]
    pub tab_type: Option<i32>,
    #[serde(rename = "voiceResourceName")]
    pub voice_resource_name: Option<String>,
}

pub struct CinematalktableTable {
    records: Vec<Cinematalktable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl CinematalktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cinematalktable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            if let Some(group_id) = record.quest_group_id {
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
    pub fn get(&self, id: i32) -> Option<&Cinematalktable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Cinematalktable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Cinematalktable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Cinematalktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
