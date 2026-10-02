// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eventcoinexchangegrouptable {
    #[serde(rename = "baseCostumeId", default)]
    pub base_costume_id: Option<i32>,
    #[serde(rename = "baseCostumeNameLocalTextId", default)]
    pub base_costume_name_local_text_id: Option<i32>,
    #[serde(rename = "baseCostumeObtainTitleLocalTextId", default)]
    pub base_costume_obtain_title_local_text_id: Option<i32>,
    #[serde(rename = "endPageId", default)]
    pub end_page_id: i32,
    #[serde(rename = "eventNameLocalTextId", default)]
    pub event_name_local_text_id: i32,
    #[serde(rename = "firstButtonCount", default)]
    pub first_button_count: i32,
    #[serde(rename = "guideDescLocalTextId", default)]
    pub guide_desc_local_text_id: Vec<i32>,
    #[serde(rename = "guideTitleLocalTextId", default)]
    pub guide_title_local_text_id: Vec<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "isRepeatableReward", default)]
    pub is_repeatable_reward: Option<i32>,
    #[serde(rename = "itemAcquireId", default)]
    pub item_acquire_id: Option<Vec<i32>>,
    #[serde(rename = "itemCount", default)]
    pub item_count: i32,
    #[serde(rename = "itemId", default)]
    pub item_id: Option<i32>,
    #[serde(rename = "itemType", default)]
    pub item_type: i32,
    #[serde(rename = "probabilityInfoLocalTextId", default)]
    pub probability_info_local_text_id: Option<i32>,
    #[serde(rename = "ratioWebLink", default)]
    pub ratio_web_link: Option<i32>,
    #[serde(rename = "secondButtonCount", default)]
    pub second_button_count: Option<i32>,
    #[serde(rename = "skinGuideDescLocalTextId", default)]
    pub skin_guide_desc_local_text_id: Option<i32>,
    #[serde(rename = "skinGuideTitleLocalTextId", default)]
    pub skin_guide_title_local_text_id: Option<i32>,
    #[serde(rename = "startPageId", default)]
    pub start_page_id: i32,
    #[serde(rename = "tokenInfoLocation", default)]
    pub token_info_location: Option<i32>,
    #[serde(rename = "tokenShortCutId", default)]
    pub token_short_cut_id: Option<i32>,
    #[serde(rename = "unlockRatioCount", default)]
    pub unlock_ratio_count: Option<i32>,
    #[serde(rename = "freeCount", default)]
    pub free_count: Option<i32>,
    #[serde(rename = "freeCountType", default)]
    pub free_count_type: Option<i32>,
}

pub struct EventcoinexchangegrouptableTable {
    records: Vec<Eventcoinexchangegrouptable>,
    by_id: HashMap<i32, usize>,
}

impl EventcoinexchangegrouptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Eventcoinexchangegrouptable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Eventcoinexchangegrouptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Eventcoinexchangegrouptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Eventcoinexchangegrouptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
