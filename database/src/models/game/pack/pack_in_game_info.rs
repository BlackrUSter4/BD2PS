use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PackInGameInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "CharInfoIndex")]
    pub char_info_index: Option<String>,
    #[sqlx(rename = "QuestInfoIndex")]
    pub quest_info_index: Option<String>,
    #[sqlx(rename = "ClearQuestIds")]
    pub clear_quest_ids: i32,
    #[sqlx(rename = "Position")]
    pub position: Option<String>,
    #[sqlx(rename = "TalentNpcInfoIndex")]
    pub talent_npc_info_index: Option<String>,
    #[sqlx(rename = "MonsterInfoIndex")]
    pub monster_info_index: Option<String>,
    #[sqlx(rename = "TalentObjectInfoIndex")]
    pub talent_object_info_index: Option<String>,
    #[sqlx(rename = "FieldBuffInfoIndex")]
    pub field_buff_info_index: Option<String>,
    #[sqlx(rename = "ReputationInfoIndex")]
    pub reputation_info_index: Option<String>,
    #[sqlx(rename = "MapActiveInfoIndex")]
    pub map_active_info_index: Option<String>,
    #[sqlx(rename = "ResearchObjectId")]
    pub research_object_id: i32,
    #[sqlx(rename = "HuntingGroundInfoIndex")]
    pub hunting_ground_info_index: Option<i64>,
    #[sqlx(rename = "TalentSkillInfoIndex")]
    pub talent_skill_info_index: Option<String>,
    #[sqlx(rename = "ContentRankStatueInfoIndex")]
    pub content_rank_statue_info_index: Option<String>,
    #[sqlx(rename = "StatueRewardObtainId")]
    pub statue_reward_obtain_id: i32,
    #[sqlx(rename = "RewardInfoBundleIndex")]
    pub reward_info_bundle_index: Option<i64>,
}