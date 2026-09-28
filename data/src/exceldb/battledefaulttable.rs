// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battledefaulttable {
    #[serde(rename = "AtkMax")]
    pub atk_max: i32,
    #[serde(rename = "CriDMax")]
    pub cri_d_max: f32,
    #[serde(rename = "CriMax")]
    pub cri_max: f32,
    #[serde(rename = "DefMax")]
    pub def_max: f32,
    #[serde(rename = "FieldObjectBattleRearrangeEffectTime")]
    pub field_object_battle_rearrange_effect_time: i32,
    #[serde(rename = "FieldObjectBattleRearrangeTime")]
    pub field_object_battle_rearrange_time: i32,
    #[serde(rename = "HpMaxLimit")]
    pub hp_max_limit: i32,
    #[serde(rename = "SpReductionMax")]
    pub sp_reduction_max: i32,
    #[serde(rename = "SpReductionMin")]
    pub sp_reduction_min: i32,
    #[serde(rename = "battleContinueCostItemCount")]
    pub battle_continue_cost_item_count: i32,
    #[serde(rename = "battleContinueCostItemType")]
    pub battle_continue_cost_item_type: i32,
    #[serde(rename = "battleContinueMaxCount")]
    pub battle_continue_max_count: i32,
    #[serde(rename = "battlePowerConst")]
    pub battle_power_const: i32,
    #[serde(rename = "chainDamageValue")]
    pub chain_damage_value: f32,
    #[serde(rename = "chainMaxCount")]
    pub chain_max_count: i32,
    #[serde(rename = "deathTimeBuffId")]
    pub death_time_buff_id: i32,
    #[serde(rename = "deathTimeStartTurnPvP")]
    pub death_time_start_turn_pv_p: i32,
    #[serde(rename = "frontMoveSec")]
    pub front_move_sec: f32,
    #[serde(rename = "minAttackDamage")]
    pub min_attack_damage: i32,
    #[serde(rename = "sideMoveSec")]
    pub side_move_sec: f32,
    #[serde(rename = "spGuildRaidMaxCount")]
    pub sp_guild_raid_max_count: i32,
    #[serde(rename = "spMaxCount")]
    pub sp_max_count: i32,
    #[serde(rename = "spStartGuildRaidCount")]
    pub sp_start_guild_raid_count: i32,
    #[serde(rename = "spStartHunterCount")]
    pub sp_start_hunter_count: Option<i32>,
    #[serde(rename = "spStartPvECount")]
    pub sp_start_pv_e_count: i32,
    #[serde(rename = "spStartPvPBLUECount")]
    pub sp_start_pv_p_b_l_u_e_count: i32,
    #[serde(rename = "spStartPvPREDCount")]
    pub sp_start_pv_p_r_e_d_count: i32,
    #[serde(rename = "spTurnAddGuildRaidCount")]
    pub sp_turn_add_guild_raid_count: i32,
    #[serde(rename = "spTurnAddHunterCount")]
    pub sp_turn_add_hunter_count: Option<i32>,
    #[serde(rename = "spTurnAddPvPCount")]
    pub sp_turn_add_pv_p_count: i32,
    #[serde(rename = "strongElementEffect")]
    pub strong_element_effect: f32,
    #[serde(rename = "turnPassSec")]
    pub turn_pass_sec: f32,
    #[serde(rename = "CriRatio")]
    pub cri_ratio: Option<i32>,
    #[serde(rename = "battleResumeLimit")]
    pub battle_resume_limit: Option<i32>,
}

pub struct BattledefaulttableTable {
    records: Vec<Battledefaulttable>,
}

impl BattledefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Battledefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Battledefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Battledefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
