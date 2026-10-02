// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rldefaulttable {
    #[serde(rename = "bgmName", default)]
    pub bgm_name: String,
    #[serde(rename = "bossRoom", default)]
    pub boss_room: Vec<i32>,
    #[serde(rename = "costumeSlot1Option", default)]
    pub costume_slot1_option: i32,
    #[serde(rename = "costumeSlot2Option", default)]
    pub costume_slot2_option: i32,
    #[serde(rename = "costumeSlot3Option", default)]
    pub costume_slot3_option: i32,
    #[serde(rename = "costumeSlot4Option", default)]
    pub costume_slot4_option: i32,
    #[serde(rename = "costumeUpgradePrice", default)]
    pub costume_upgrade_price: i32,
    #[serde(rename = "entryBuyPrice", default)]
    pub entry_buy_price: i32,
    #[serde(rename = "floorCount", default)]
    pub floor_count: i32,
    #[serde(rename = "floorId", default)]
    pub floor_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "memberChangeSP", default)]
    pub member_change_s_p: i32,
    #[serde(rename = "roguelikeApCount", default)]
    pub roguelike_ap_count: i32,
    #[serde(rename = "roguelikeGrowthCurrency", default)]
    pub roguelike_growth_currency: i32,
    #[serde(rename = "roomRatio", default)]
    pub room_ratio: i32,
    #[serde(rename = "seasonDefaultLostGold", default)]
    pub season_default_lost_gold: i32,
    #[serde(rename = "shopDiscountRate", default)]
    pub shop_discount_rate: f32,
    #[serde(rename = "shopHealPrice", default)]
    pub shop_heal_price: i32,
    #[serde(rename = "shopRerollExpensive", default)]
    pub shop_reroll_expensive: i32,
    #[serde(rename = "shopRerollPrice", default)]
    pub shop_reroll_price: i32,
    #[serde(rename = "startCostumeCount", default)]
    pub start_costume_count: i32,
    #[serde(rename = "startGold", default)]
    pub start_gold: i32,
}

pub struct RldefaulttableTable {
    records: Vec<Rldefaulttable>,
    by_id: HashMap<i32, usize>,
}

impl RldefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rldefaulttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Rldefaulttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rldefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rldefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
