use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct BattleSession {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "BattleIndex")]
    pub battle_index: Option<i32>,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    #[sqlx(rename = "MonsterId")]
    pub monster_id: Option<i32>,
    #[sqlx(rename = "PackId")]
    pub pack_id: Option<i32>,
    #[sqlx(rename = "BattleDeck")]
    pub battle_deck: Option<i32>,
    #[sqlx(rename = "BattleMode")]
    pub battle_mode: Option<i32>,
    #[sqlx(rename = "MonsterHuntId")]
    pub monster_hunt_id: Option<i32>,
    #[sqlx(rename = "StageMagicGroupId")]
    pub stage_magic_group_id: Option<i32>,
    #[sqlx(rename = "StageMagicId")]
    pub stage_magic_id: Option<i32>,
    #[sqlx(rename = "StageMagicLevel")]
    pub stage_magic_level: Option<i32>,
    #[sqlx(rename = "RandomSeed")]
    pub random_seed: Option<i32>,
    #[sqlx(rename = "CreatedAt")]
    pub created_at: i64,
}
