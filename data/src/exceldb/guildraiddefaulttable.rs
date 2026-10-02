// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guildraiddefaulttable {
    #[serde(rename = "chainPoint", default)]
    pub chain_point: f32,
    #[serde(rename = "charBaseScore", default)]
    pub char_base_score: f32,
    #[serde(rename = "dailyUseSupporter", default)]
    pub daily_use_supporter: i32,
    #[serde(rename = "getBossHpPoint", default)]
    pub get_boss_hp_point: f32,
    #[serde(rename = "getDamagePoint", default)]
    pub get_damage_point: f32,
    #[serde(rename = "golemCharId", default)]
    pub golem_char_id: i32,
    #[serde(rename = "golemRemainTurn", default)]
    pub golem_remain_turn: i32,
    #[serde(rename = "golemResource", default)]
    pub golem_resource: String,
    #[serde(rename = "golemScoreValue", default)]
    pub golem_score_value: f32,
    #[serde(rename = "iconSpriteName", default)]
    pub icon_sprite_name: String,
    #[serde(rename = "itemDescNameTextId", default)]
    pub item_desc_name_text_id: i32,
    #[serde(rename = "itemNameTextId", default)]
    pub item_name_text_id: i32,
    #[serde(rename = "itemSubNameTextId", default)]
    pub item_sub_name_text_id: i32,
    #[serde(rename = "mailId", default)]
    pub mail_id: i32,
    #[serde(rename = "maxGolemGauge", default)]
    pub max_golem_gauge: f32,
    #[serde(rename = "phaseValue", default)]
    pub phase_value: f32,
    #[serde(rename = "raidBattleDailyEncount", default)]
    pub raid_battle_daily_encount: i32,
    #[serde(rename = "seasonNumber", default)]
    pub season_number: i32,
    #[serde(rename = "statuePackId", default)]
    pub statue_pack_id: i32,
    #[serde(rename = "supportCharBasicRewardCount", default)]
    pub support_char_basic_reward_count: i32,
    #[serde(rename = "supportCharRentalRewardCount", default)]
    pub support_char_rental_reward_count: i32,
    #[serde(rename = "supportScore", default)]
    pub support_score: f32,
    #[serde(rename = "turnAddScore", default)]
    pub turn_add_score: f32,
    #[serde(rename = "turnScoreValue", default)]
    pub turn_score_value: f32,
}

pub struct GuildraiddefaulttableTable {
    records: Vec<Guildraiddefaulttable>,
}

impl GuildraiddefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Guildraiddefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Guildraiddefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Guildraiddefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
