// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gachagrouptable {
    #[serde(rename = "bannerFontLocalTextId", default)]
    pub banner_font_local_text_id: Option<i32>,
    #[serde(rename = "buyLimitCount", default)]
    pub buy_limit_count: Option<i32>,
    #[serde(rename = "descLocalTextId", default)]
    pub desc_local_text_id: i32,
    #[serde(rename = "endGetItemCount", default)]
    pub end_get_item_count: Option<i32>,
    #[serde(rename = "endGetItemType", default)]
    pub end_get_item_type: Option<i32>,
    #[serde(rename = "fixedId", default)]
    pub fixed_id: Option<i32>,
    #[serde(rename = "gachaBannerBg", default)]
    pub gacha_banner_bg: String,
    #[serde(rename = "gachaBannerImage", default)]
    pub gacha_banner_image: String,
    #[serde(rename = "gachaNameTextId", default)]
    pub gacha_name_text_id: i32,
    #[serde(rename = "gachaSpineBg", default)]
    pub gacha_spine_bg: Option<String>,
    #[serde(rename = "gachaSubType", default)]
    pub gacha_sub_type: Option<i32>,
    #[serde(rename = "gachaType", default)]
    pub gacha_type: Option<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "isDisplayUiOwnGachaTicket", default)]
    pub is_display_ui_own_gacha_ticket: Option<i32>,
    #[serde(rename = "isPickUpExchange", default)]
    pub is_pick_up_exchange: Option<i32>,
    #[serde(rename = "isSelectedFromPity", default)]
    pub is_selected_from_pity: Option<i32>,
    #[serde(rename = "isSpecialSelectionGacha", default)]
    pub is_special_selection_gacha: Option<i32>,
    #[serde(rename = "oneTimeGachaId", default)]
    pub one_time_gacha_id: Option<i32>,
    #[serde(rename = "pickUpExchangeCost", default)]
    pub pick_up_exchange_cost: Option<i32>,
    #[serde(rename = "pickUpItemId", default)]
    pub pick_up_item_id: Option<i32>,
    #[serde(rename = "pointCount", default)]
    pub point_count: Option<i32>,
    #[serde(rename = "scheduleType", default)]
    pub schedule_type: Option<i32>,
    #[serde(rename = "selectCount", default)]
    pub select_count: Option<i32>,
    #[serde(rename = "selectionChangeCount", default)]
    pub selection_change_count: Option<i32>,
    #[serde(rename = "selectionChoiceRate", default)]
    pub selection_choice_rate: Option<i32>,
    #[serde(rename = "sortId", default)]
    pub sort_id: Option<i32>,
    #[serde(rename = "tenTimeGachaId", default)]
    pub ten_time_gacha_id: i32,
    #[serde(rename = "useAccumulateRateLog", default)]
    pub use_accumulate_rate_log: Option<i32>,
    #[serde(rename = "useGachaTicketOption", default)]
    pub use_gacha_ticket_option: Option<i32>,
    #[serde(rename = "useSelectionOnlyFixedApply", default)]
    pub use_selection_only_fixed_apply: Option<i32>,
    #[serde(rename = "cashProductGroupId", default)]
    pub cash_product_group_id: Option<i32>,
    #[serde(rename = "cashProductId", default)]
    pub cash_product_id: Option<i32>,
    #[serde(rename = "gachaLocalText", default)]
    pub gacha_local_text: Option<i32>,
    #[serde(rename = "isShowGachaRateWhenConditionSatisfied", default)]
    pub is_show_gacha_rate_when_condition_satisfied: Option<i32>,
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
    pub fn iter(&self) -> std::slice::Iter<'_, Gachagrouptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
