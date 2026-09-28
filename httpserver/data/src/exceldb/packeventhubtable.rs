// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packeventhubtable {
    #[serde(rename = "archivingSortId")]
    pub archiving_sort_id: i32,
    #[serde(rename = "atlasName")]
    pub atlas_name: String,
    #[serde(rename = "bgmName")]
    pub bgm_name: String,
    #[serde(rename = "collaboId")]
    pub collabo_id: Option<i32>,
    #[serde(rename = "contentGenreId")]
    pub content_genre_id: Option<Vec<i32>>,
    #[serde(rename = "eventBgIllustName")]
    pub event_bg_illust_name: String,
    #[serde(rename = "eventLogoName")]
    pub event_logo_name: String,
    #[serde(rename = "eventNameTextId")]
    pub event_name_text_id: i32,
    #[serde(rename = "eventResourceId")]
    pub event_resource_id: i32,
    #[serde(rename = "fakePackId")]
    pub fake_pack_id: i32,
    #[serde(rename = "guideDescTextId")]
    pub guide_desc_text_id: Vec<i32>,
    #[serde(rename = "guideTitleTextId")]
    pub guide_title_text_id: Vec<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isHUD")]
    pub is_h_u_d: Option<i32>,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "packBannerName")]
    pub pack_banner_name: String,
    #[serde(rename = "packCoverName")]
    pub pack_cover_name: Option<String>,
    #[serde(rename = "packId")]
    pub pack_id: i32,
    #[serde(rename = "storyGroupId")]
    pub story_group_id: Option<i32>,
    #[serde(rename = "storySynopsisQuestTextId")]
    pub story_synopsis_quest_text_id: Option<i32>,
}

pub struct PackeventhubtableTable {
    records: Vec<Packeventhubtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl PackeventhubtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Packeventhubtable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            if let Some(group_id) = record.story_group_id {
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
    pub fn get(&self, id: i32) -> Option<&Packeventhubtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Packeventhubtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Packeventhubtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Packeventhubtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
