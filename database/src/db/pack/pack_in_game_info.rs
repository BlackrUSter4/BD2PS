use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::pack::pack_in_game_info::PackInGameInfo;
/// Insert a full JSON array of PackInGameInfo records for a UID.
pub async fn insert_pack_in_game_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("packInGameInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_pack_in_game_info: missing or invalid 'packInGameInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle parallel repeated arrays - iterate all together
        let char_info_index = entry
            .get("charInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let quest_info_index = entry
            .get("questInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let position = entry
            .get("position")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let talent_npc_info_index = entry
            .get("talentNpcInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let monster_info_index = entry
            .get("monsterInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let talent_object_info_index = entry
            .get("talentObjectInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let field_buff_info_index = entry
            .get("fieldBuffInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let reputation_info_index = entry
            .get("reputationInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let map_active_info_index = entry
            .get("mapActiveInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let hunting_ground_info_index = entry
            .get("huntingGroundInfoIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let talent_skill_info_index = entry
            .get("talentSkillInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let content_rank_statue_info_index = entry
            .get("contentRankStatueInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let reward_info_bundle_index = entry
            .get("rewardInfoBundleIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        // Get all parallel arrays
        let clear_quest_ids_array = entry.get("clearQuestIds").and_then(|v| v.as_array());
        let research_object_id_array = entry.get("researchObjectId").and_then(|v| v.as_array());
        let statue_reward_obtain_id_array = entry.get("statueRewardObtainId").and_then(|v| v.as_array());

        // Determine max length
        let len = 0
            .max(clear_quest_ids_array.map(|a| a.len()).unwrap_or(0))
            .max(research_object_id_array.map(|a| a.len()).unwrap_or(0))
            .max(statue_reward_obtain_id_array.map(|a| a.len()).unwrap_or(0));

        if len > 0 {
            // Insert one row per index (parallel iteration)
            for i in 0..len {
                let clear_quest_ids = clear_quest_ids_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                let research_object_id = research_object_id_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                let statue_reward_obtain_id = statue_reward_obtain_id_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;

                sqlx::query(
                    r#"
INSERT INTO PackInGameInfo (
    Uid,
    CharInfoIndex,
    QuestInfoIndex,
    ClearQuestIds,
    Position,
    TalentNpcInfoIndex,
    MonsterInfoIndex,
    TalentObjectInfoIndex,
    FieldBuffInfoIndex,
    ReputationInfoIndex,
    MapActiveInfoIndex,
    ResearchObjectId,
    HuntingGroundInfoIndex,
    TalentSkillInfoIndex,
    ContentRankStatueInfoIndex,
    StatueRewardObtainId,
    RewardInfoBundleIndex
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
                )
                .bind(uid)
                .bind(&char_info_index)
                .bind(&quest_info_index)
                .bind(clear_quest_ids)
                .bind(&position)
                .bind(&talent_npc_info_index)
                .bind(&monster_info_index)
                .bind(&talent_object_info_index)
                .bind(&field_buff_info_index)
                .bind(&reputation_info_index)
                .bind(&map_active_info_index)
                .bind(research_object_id)
                .bind(&hunting_ground_info_index)
                .bind(&talent_skill_info_index)
                .bind(&content_rank_statue_info_index)
                .bind(statue_reward_obtain_id)
                .bind(&reward_info_bundle_index)
                .execute(pool)
                .await?;
            }
        } else {
            // No items, insert one row with NULLs for repeated fields
            sqlx::query(
                r#"
INSERT INTO PackInGameInfo (
    Uid,
    CharInfoIndex,
    QuestInfoIndex,
    ClearQuestIds,
    Position,
    TalentNpcInfoIndex,
    MonsterInfoIndex,
    TalentObjectInfoIndex,
    FieldBuffInfoIndex,
    ReputationInfoIndex,
    MapActiveInfoIndex,
    ResearchObjectId,
    HuntingGroundInfoIndex,
    TalentSkillInfoIndex,
    ContentRankStatueInfoIndex,
    StatueRewardObtainId,
    RewardInfoBundleIndex
) VALUES (
    ?,
    ?,
    ?,
    NULL,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    NULL,
    ?,
    ?,
    ?,
    NULL,
    ?
)
"#
            )
            .bind(uid)
            .bind(&char_info_index)
            .bind(&quest_info_index)
            .bind(&position)
            .bind(&talent_npc_info_index)
            .bind(&monster_info_index)
            .bind(&talent_object_info_index)
            .bind(&field_buff_info_index)
            .bind(&reputation_info_index)
            .bind(&map_active_info_index)
            .bind(&hunting_ground_info_index)
            .bind(&talent_skill_info_index)
            .bind(&content_rank_statue_info_index)
            .bind(&reward_info_bundle_index)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

/// Add a single PackInGameInfo record from a Rust struct.
pub async fn add_pack_in_game_info(pool: &SqlitePool, data: &PackInGameInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PackInGameInfo (
    Uid,
    CharInfoIndex,
    QuestInfoIndex,
    ClearQuestIds,
    Position,
    TalentNpcInfoIndex,
    MonsterInfoIndex,
    TalentObjectInfoIndex,
    FieldBuffInfoIndex,
    ReputationInfoIndex,
    MapActiveInfoIndex,
    ResearchObjectId,
    HuntingGroundInfoIndex,
    TalentSkillInfoIndex,
    ContentRankStatueInfoIndex,
    StatueRewardObtainId,
    RewardInfoBundleIndex
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.char_info_index)
    .bind(&data.quest_info_index)
    .bind(&data.clear_quest_ids)
    .bind(&data.position)
    .bind(&data.talent_npc_info_index)
    .bind(&data.monster_info_index)
    .bind(&data.talent_object_info_index)
    .bind(&data.field_buff_info_index)
    .bind(&data.reputation_info_index)
    .bind(&data.map_active_info_index)
    .bind(&data.research_object_id)
    .bind(&data.hunting_ground_info_index)
    .bind(&data.talent_skill_info_index)
    .bind(&data.content_rank_statue_info_index)
    .bind(&data.statue_reward_obtain_id)
    .bind(&data.reward_info_bundle_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pack_in_game_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<PackInGameInfo>> {
    sqlx::query_as::<_, PackInGameInfo>("SELECT * FROM PackInGameInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PackInGameInfo rows for a UID.
pub async fn delete_pack_in_game_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PackInGameInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}