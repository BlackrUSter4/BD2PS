// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visualnoveldialogtable {
    #[serde(rename = "VoiceResourceName")]
    pub voice_resource_name: Option<String>,
    #[serde(rename = "bgIllust")]
    pub bg_illust: Option<String>,
    #[serde(rename = "cameraFocusNum")]
    pub camera_focus_num: Option<i32>,
    #[serde(rename = "charAction")]
    pub char_action: Vec<i32>,
    #[serde(rename = "charFace")]
    pub char_face: Vec<String>,
    #[serde(rename = "costumeDesignId")]
    pub costume_design_id: Vec<i32>,
    #[serde(rename = "faceIllustName")]
    pub face_illust_name: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "nameTextId")]
    pub name_text_id: Option<i32>,
    #[serde(rename = "selectDialogId")]
    pub select_dialog_id: Option<i32>,
    #[serde(rename = "soundEventId")]
    pub sound_event_id: Option<i32>,
    #[serde(rename = "specialEvent")]
    pub special_event: Option<i32>,
    #[serde(rename = "specialIllust")]
    pub special_illust: Option<String>,
    #[serde(rename = "storyTextId")]
    pub story_text_id: Option<i32>,
    #[serde(rename = "talkCharNum")]
    pub talk_char_num: i32,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
}

pub struct VisualnoveldialogtableTable {
    records: Vec<Visualnoveldialogtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl VisualnoveldialogtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Visualnoveldialogtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Visualnoveldialogtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Visualnoveldialogtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Visualnoveldialogtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Visualnoveldialogtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
