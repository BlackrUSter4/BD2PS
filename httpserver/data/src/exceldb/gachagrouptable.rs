// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gachagrouptable {
    #[serde(rename = "bannerFontLocalTextId")]
    pub banner_font_local_text_id: Option<i32>,
    #[serde(rename = "buyLimitCount")]
    pub buy_limit_count: Option<i32>,
    #[serde(rename = "descLocalTextId")]
    pub desc_local_text_id: i32,
    #[serde(rename = "endGetItemCount")]
    pub end_get_item_count: Option<i32>,
    #[serde(rename = "endGetItemType")]
    pub end_get_item_type: Option<i32>,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<i32>,
    #[serde(rename = "gachaBannerBg")]
    pub gacha_banner_bg: String,
    #[serde(rename = "gachaBannerImage")]
    pub gacha_banner_image: String,
    #[serde(rename = "gachaNameTextId")]
    pub gacha_name_text_id: i32,
    #[serde(rename = "gachaSpineBg")]
    pub gacha_spine_bg: Option<String>,
    #[serde(rename = "gachaSubType")]
    pub gacha_sub_type: Option<i32>,
    #[serde(rename = "gachaType")]
    pub gacha_type: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isDisplayUiOwnGachaTicket")]
    pub is_display_ui_own_gacha_ticket: Option<i32>,
    #[serde(rename = "isPickUpExchange")]
    pub is_pick_up_exchange: Option<i32>,
    #[serde(rename = "isSelectedFromPity")]
    pub is_selected_from_pity: Option<i32>,
    #[serde(rename = "isSpecialSelectionGacha")]
    pub is_special_selection_gacha: Option<i32>,
    #[serde(rename = "oneTimeGachaId")]
    pub one_time_gacha_id: Option<i32>,
    #[serde(rename = "pickUpExchangeCost")]
    pub pick_up_exchange_cost: Option<i32>,
    #[serde(rename = "pickUpItemId")]
    pub pick_up_item_id: Option<i32>,
    #[serde(rename = "pointCount")]
    pub point_count: Option<i32>,
    #[serde(rename = "scheduleType")]
    pub schedule_type: Option<i32>,
    #[serde(rename = "selectCount")]
    pub select_count: Option<i32>,
    #[serde(rename = "selectionChangeCount")]
    pub selection_change_count: Option<i32>,
    #[serde(rename = "selectionChoiceRate")]
    pub selection_choice_rate: Option<i32>,
    #[serde(rename = "sortId")]
    pub sort_id: Option<i32>,
    #[serde(rename = "tenTimeGachaId")]
    pub ten_time_gacha_id: i32,
    #[serde(rename = "useAccumulateRateLog")]
    pub use_accumulate_rate_log: Option<i32>,
    #[serde(rename = "useGachaTicketOption")]
    pub use_gacha_ticket_option: Option<i32>,
    #[serde(rename = "useSelectionOnlyFixedApply")]
    pub use_selection_only_fixed_apply: Option<i32>,
}

pub struct GachagrouptableTable {
    records: Vec<Gachagrouptable>,
    by_id: HashMap<i32, usize>,
}

impl GachagrouptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Gachagrouptable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Gachagrouptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Gachagrouptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Gachagrouptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
