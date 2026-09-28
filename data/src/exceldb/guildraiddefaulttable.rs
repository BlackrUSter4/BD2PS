// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guildraiddefaulttable {
    #[serde(rename = "chainPoint")]
    pub chain_point: f32,
    #[serde(rename = "charBaseScore")]
    pub char_base_score: f32,
    #[serde(rename = "dailyUseSupporter")]
    pub daily_use_supporter: i32,
    #[serde(rename = "getBossHpPoint")]
    pub get_boss_hp_point: f32,
    #[serde(rename = "getDamagePoint")]
    pub get_damage_point: f32,
    #[serde(rename = "golemCharId")]
    pub golem_char_id: i32,
    #[serde(rename = "golemRemainTurn")]
    pub golem_remain_turn: i32,
    #[serde(rename = "golemResource")]
    pub golem_resource: String,
    #[serde(rename = "golemScoreValue")]
    pub golem_score_value: f32,
    #[serde(rename = "iconSpriteName")]
    pub icon_sprite_name: String,
    #[serde(rename = "itemDescNameTextId")]
    pub item_desc_name_text_id: i32,
    #[serde(rename = "itemNameTextId")]
    pub item_name_text_id: i32,
    #[serde(rename = "itemSubNameTextId")]
    pub item_sub_name_text_id: i32,
    #[serde(rename = "mailId")]
    pub mail_id: i32,
    #[serde(rename = "maxGolemGauge")]
    pub max_golem_gauge: f32,
    #[serde(rename = "phaseValue")]
    pub phase_value: f32,
    #[serde(rename = "raidBattleDailyEncount")]
    pub raid_battle_daily_encount: i32,
    #[serde(rename = "seasonNumber")]
    pub season_number: i32,
    #[serde(rename = "statuePackId")]
    pub statue_pack_id: i32,
    #[serde(rename = "supportCharBasicRewardCount")]
    pub support_char_basic_reward_count: i32,
    #[serde(rename = "supportCharRentalRewardCount")]
    pub support_char_rental_reward_count: i32,
    #[serde(rename = "supportScore")]
    pub support_score: f32,
    #[serde(rename = "turnAddScore")]
    pub turn_add_score: f32,
    #[serde(rename = "turnScoreValue")]
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
