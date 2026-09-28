// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eventcoinexchangegrouptable {
    #[serde(rename = "baseCostumeId")]
    pub base_costume_id: Option<i32>,
    #[serde(rename = "baseCostumeNameLocalTextId")]
    pub base_costume_name_local_text_id: Option<i32>,
    #[serde(rename = "baseCostumeObtainTitleLocalTextId")]
    pub base_costume_obtain_title_local_text_id: Option<i32>,
    #[serde(rename = "endPageId")]
    pub end_page_id: i32,
    #[serde(rename = "eventNameLocalTextId")]
    pub event_name_local_text_id: i32,
    #[serde(rename = "firstButtonCount")]
    pub first_button_count: i32,
    #[serde(rename = "guideDescLocalTextId")]
    pub guide_desc_local_text_id: Vec<i32>,
    #[serde(rename = "guideTitleLocalTextId")]
    pub guide_title_local_text_id: Vec<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isRepeatableReward")]
    pub is_repeatable_reward: Option<i32>,
    #[serde(rename = "itemAcquireId")]
    pub item_acquire_id: Option<Vec<i32>>,
    #[serde(rename = "itemCount")]
    pub item_count: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "itemType")]
    pub item_type: i32,
    #[serde(rename = "probabilityInfoLocalTextId")]
    pub probability_info_local_text_id: Option<i32>,
    #[serde(rename = "ratioWebLink")]
    pub ratio_web_link: Option<i32>,
    #[serde(rename = "secondButtonCount")]
    pub second_button_count: Option<i32>,
    #[serde(rename = "skinGuideDescLocalTextId")]
    pub skin_guide_desc_local_text_id: Option<i32>,
    #[serde(rename = "skinGuideTitleLocalTextId")]
    pub skin_guide_title_local_text_id: Option<i32>,
    #[serde(rename = "startPageId")]
    pub start_page_id: i32,
    #[serde(rename = "tokenInfoLocation")]
    pub token_info_location: Option<i32>,
    #[serde(rename = "tokenShortCutId")]
    pub token_short_cut_id: Option<i32>,
    #[serde(rename = "unlockRatioCount")]
    pub unlock_ratio_count: Option<i32>,
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
    pub fn iter(&self) -> std::slice::Iter<Eventcoinexchangegrouptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
