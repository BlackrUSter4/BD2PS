// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packeventhubtable {
    #[serde(rename = "archivingSortId", default)]
    pub archiving_sort_id: i32,
    #[serde(rename = "atlasName", default)]
    pub atlas_name: String,
    #[serde(rename = "bgmName", default)]
    pub bgm_name: String,
    #[serde(rename = "collaboId", default)]
    pub collabo_id: Option<i32>,
    #[serde(rename = "contentGenreId", default)]
    pub content_genre_id: Option<Vec<i32>>,
    #[serde(rename = "eventBgIllustName", default)]
    pub event_bg_illust_name: String,
    #[serde(rename = "eventLogoName", default)]
    pub event_logo_name: String,
    #[serde(rename = "eventNameTextId", default)]
    pub event_name_text_id: i32,
    #[serde(rename = "eventResourceId", default)]
    pub event_resource_id: i32,
    #[serde(rename = "fakePackId", default)]
    pub fake_pack_id: i32,
    #[serde(rename = "guideDescTextId", default)]
    pub guide_desc_text_id: Vec<i32>,
    #[serde(rename = "guideTitleTextId", default)]
    pub guide_title_text_id: Vec<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "isHUD", default)]
    pub is_h_u_d: Option<i32>,
    #[serde(rename = "label", default)]
    pub label: String,
    #[serde(rename = "packBannerName", default)]
    pub pack_banner_name: String,
    #[serde(rename = "packCoverName", default)]
    pub pack_cover_name: Option<String>,
    #[serde(rename = "packId", default)]
    pub pack_id: i32,
    #[serde(rename = "storyGroupId", default)]
    pub story_group_id: Option<i32>,
    #[serde(rename = "storySynopsisQuestTextId", default)]
    pub story_synopsis_quest_text_id: Option<i32>,
    #[serde(rename = "hubType", default)]
    pub hub_type: Option<i32>,
    #[serde(rename = "loadingPageName", default)]
    pub loading_page_name: Option<String>,
}

pub struct PackeventhubtableTable {
    records: Vec<Packeventhubtable>,
    by_id: HashMap<i32, usize>,
}

impl PackeventhubtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Packeventhubtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Packeventhubtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Packeventhubtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Packeventhubtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
