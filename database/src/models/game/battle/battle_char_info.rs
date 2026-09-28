use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct BattleCharInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "UniqueIndex")]
    pub unique_index: Option<i32>,
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: Option<i64>,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "Hp")]
    pub hp: Option<i64>,
    #[sqlx(rename = "Level")]
    pub level: Option<i32>,
    #[sqlx(rename = "CostumeInvenIndex")]
    pub costume_inven_index: Option<i64>,
    #[sqlx(rename = "CostumeId")]
    pub costume_id: Option<i32>,
    #[sqlx(rename = "CostumeLevel")]
    pub costume_level: Option<i32>,
    #[sqlx(rename = "GridIndex")]
    pub grid_index: Option<i32>,
    #[sqlx(rename = "ReserveCostumeId")]
    pub reserve_costume_id: Option<i32>,
    #[sqlx(rename = "IsActiveSubSkillUse")]
    pub is_active_sub_skill_use: Option<bool>,
    #[sqlx(rename = "BuffPlusStat")]
    pub buff_plus_stat: Option<String>,
    #[sqlx(rename = "BuffMultipleStat")]
    pub buff_multiple_stat: Option<String>,
    #[sqlx(rename = "AttackDamage")]
    pub attack_damage: Option<i64>,
    #[sqlx(rename = "BattlePower")]
    pub battle_power: Option<i32>,
    #[sqlx(rename = "TotalWarPlayType")]
    pub total_war_play_type: Option<i32>,
    #[sqlx(rename = "ConnectPotentialCostume")]
    pub connect_potential_costume: Option<i32>,
    #[sqlx(rename = "Key")]
    pub key: i32,
    #[sqlx(rename = "Value")]
    pub value: i32,
    #[sqlx(rename = "TargetingCount")]
    pub targeting_count: Option<i32>,
    #[sqlx(rename = "SupporterOwnerIndex")]
    pub supporter_owner_index: Option<i64>,
    #[sqlx(rename = "SupporterSlotIndex")]
    pub supporter_slot_index: Option<i32>,
    #[sqlx(rename = "CostumeDesignId")]
    pub costume_design_id: Option<i32>,
}