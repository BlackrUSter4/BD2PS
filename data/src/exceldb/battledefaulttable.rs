// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battledefaulttable {
    #[serde(rename = "AtkMax", default)]
    pub atk_max: i32,
    #[serde(rename = "CriDMax", default)]
    pub cri_d_max: f32,
    #[serde(rename = "CriMax", default)]
    pub cri_max: f32,
    #[serde(rename = "DefMax", default)]
    pub def_max: f32,
    #[serde(rename = "FieldObjectBattleRearrangeEffectTime", default)]
    pub field_object_battle_rearrange_effect_time: i32,
    #[serde(rename = "FieldObjectBattleRearrangeTime", default)]
    pub field_object_battle_rearrange_time: i32,
    #[serde(rename = "HpMaxLimit", default)]
    pub hp_max_limit: i32,
    #[serde(rename = "SpReductionMax", default)]
    pub sp_reduction_max: i32,
    #[serde(rename = "SpReductionMin", default)]
    pub sp_reduction_min: i32,
    #[serde(rename = "battleContinueCostItemCount", default)]
    pub battle_continue_cost_item_count: i32,
    #[serde(rename = "battleContinueCostItemType", default)]
    pub battle_continue_cost_item_type: i32,
    #[serde(rename = "battleContinueMaxCount", default)]
    pub battle_continue_max_count: i32,
    #[serde(rename = "battlePowerConst", default)]
    pub battle_power_const: i32,
    #[serde(rename = "chainDamageValue", default)]
    pub chain_damage_value: f32,
    #[serde(rename = "chainMaxCount", default)]
    pub chain_max_count: i32,
    #[serde(rename = "deathTimeBuffId", default)]
    pub death_time_buff_id: i32,
    #[serde(rename = "deathTimeStartTurnPvP", default)]
    pub death_time_start_turn_pv_p: i32,
    #[serde(rename = "frontMoveSec", default)]
    pub front_move_sec: f32,
    #[serde(rename = "minAttackDamage", default)]
    pub min_attack_damage: i32,
    #[serde(rename = "sideMoveSec", default)]
    pub side_move_sec: f32,
    #[serde(rename = "spGuildRaidMaxCount", default)]
    pub sp_guild_raid_max_count: i32,
    #[serde(rename = "spMaxCount", default)]
    pub sp_max_count: i32,
    #[serde(rename = "spStartGuildRaidCount", default)]
    pub sp_start_guild_raid_count: i32,
    #[serde(rename = "spStartHunterCount", default)]
    pub sp_start_hunter_count: Option<i32>,
    #[serde(rename = "spStartPvECount", default)]
    pub sp_start_pv_e_count: i32,
    #[serde(rename = "spStartPvPBLUECount", default)]
    pub sp_start_pv_p_b_l_u_e_count: i32,
    #[serde(rename = "spStartPvPREDCount", default)]
    pub sp_start_pv_p_r_e_d_count: i32,
    #[serde(rename = "spTurnAddGuildRaidCount", default)]
    pub sp_turn_add_guild_raid_count: i32,
    #[serde(rename = "spTurnAddHunterCount", default)]
    pub sp_turn_add_hunter_count: Option<i32>,
    #[serde(rename = "spTurnAddPvPCount", default)]
    pub sp_turn_add_pv_p_count: i32,
    #[serde(rename = "strongElementEffect", default)]
    pub strong_element_effect: f32,
    #[serde(rename = "turnPassSec", default)]
    pub turn_pass_sec: f32,
    #[serde(rename = "CriRatio", default)]
    pub cri_ratio: Option<i32>,
    #[serde(rename = "battleResumeLimit", default)]
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
