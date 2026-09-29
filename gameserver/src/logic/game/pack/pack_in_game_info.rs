use bd2::prost::Message;
use bd2::proto::proto_net::{
    ContentRankStatueDbInfo, HuntingGroundDbInfo, MapActiveInfo, MonsterDbInfo, Notify,
    PackInGameInfoRequest, PackInGameInfoResponse, QuestDbInfo, ReputationDbInfo,
    RewardDbInfoBundle, TalentSkillDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    content::content_rank_statue_info::get_content_rank_statue_info,
    hunting::hunting_ground_info::get_hunting_ground_info,
    hunting::hunting_ground_monster::get_monsters_for_hunting_ground,
    map::map_active_info::get_map_active_info,
    reputation::reputation_info::get_reputation_info,
    talent::talent_skill_info::get_talent_skill_info,
    user::user_quest as user_quest_db,
};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: PackInGameInfoRequest) -> GameResponse {
    info!("Handling PackInGameInfoRequest: {:?}", req);

    // --- 1. Fetch all user data safely ---
    // CORRECTION (2026-09-29): this used to call quest::quest_info::get_quest_info, which reads
    // a completely different, unrelated "QuestInfo" table that nothing ever writes to (always
    // empty) -- real per-account quest progress lives in UserQuest (the same table
    // QuestAccept/QuestUpdate/QuestClear/clear_quest_ids below all use). That bug meant this
    // response's quest_info always fell through to the hardcoded `id: Some(2)` placeholder
    // below, regardless of real progress -- which is also the reason the client's on-screen
    // quest tracker (GameFieldDefaultUI.ProgressInfo, populated via
    // PackManager.Enter(questInfoList, ...) from exactly this field) never showed anything real:
    // the client only displays a quest whose own QuestTable1.packId matches the pack currently
    // being entered, and the placeholder id=2 only accidentally matched pack1's own quest 2 by
    // coincidence -- for any other pack (or once quest 2 was actually cleared for real) it
    // silently showed nothing. Only IN-PROGRESS (status=1) quests belong in the tracker -- a
    // cleared (status=3) quest has already been reported via clear_quest_ids below.
    let quest_rows: Vec<database::models::game::user::user_quest::UserQuest> = match req.pack_id {
        Some(pack_id) => user_quest_db::get_by_uid_and_pack(pool, uid, pack_id)
            .await
            .unwrap_or_default(),
        None => user_quest_db::get_all_by_uid(pool, uid)
            .await
            .unwrap_or_default(),
    }
    .into_iter()
    .filter(|r| r.status == 1)
    .collect();
    let rep_rows = get_reputation_info(pool, uid).await.unwrap_or_default();

    let map_rows = match get_map_active_info(pool, uid).await {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("get_map_active_info error: {:?}", err);
            vec![]
        }
    };

    let hunting_rows = get_hunting_ground_info(pool, uid).await.unwrap_or_default();
    let talent_skill_rows = get_talent_skill_info(pool, uid).await.unwrap_or_default();
    let statue_rows = get_content_rank_statue_info(pool, uid)
        .await
        .unwrap_or_default();

    // Fetch last saved user position, or fallback to default. CORRECTION: an earlier session
    // changed this fallback to a pack21 position believing pack21 was "Knight of Blood" -- the
    // user directly confirmed Knight of Blood is actually pack1 (our internal pack ID 1, the main
    // starting story), and pack21 is a much later pack. Reverted to pack1's original default.
    let position = sqlx::query_scalar::<_, Option<String>>(
        "SELECT PackPosition FROM UserPosition WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
    .unwrap_or(None)
    .flatten()
    .unwrap_or_else(|| {
        "{\"MapId\":1,\"PlayerPosition\":{\"x\":17.8,\"y\":0.2,\"z\":-4.0},\"ColleaguePositions\":null}".to_string()
    });

    // --- 2. Transform DB rows → proto structures ---

    //  2.1 char_info — should be empty at world load
    let char_info = vec![];

    //  2.2 quest_info — real in-progress UserQuest rows for this pack. Honestly empty when the
    // account genuinely has none in progress (matches this codebase's convention of not
    // fabricating placeholder data), rather than the old hardcoded `id: Some(2)` fallback.
    let quest_info = quest_rows
        .into_iter()
        .map(|q| QuestDbInfo {
            id: Some(q.quest_id as i32),
            value: Some(q.progress),
            object_id: vec![],
            quest_level: None,
            quest_opt: None,
        })
        .collect();

    //  2.3 reputation_info — must exist (fallback if DB empty)
    let reputation_info = if rep_rows.is_empty() {
        vec![ReputationDbInfo {
            group_id: Some(1),
            state: Some(1),
            elapsed_seconds: Some(26012614),
        }]
    } else {
        rep_rows
            .into_iter()
            .map(|r| ReputationDbInfo {
                group_id: r.group_id,
                state: r.state,
                elapsed_seconds: r.elapsed_seconds,
            })
            .collect()
    };

    //  2.4 map_active_info — should default to two maps w/ 8×"ff"
    let map_active_info = map_rows
        .into_iter()
        .map(|row| {
            let active_info = serde_json::from_str::<Vec<String>>(&row.active_info)
                .unwrap_or_else(|_| vec!["ff".to_string(); 8]);

            MapActiveInfo {
                map_id: row.map_id,
                active_info,
            }
        })
        .collect::<Vec<_>>();

    // fallback default if no records exist
    let map_active_info = if map_active_info.is_empty() {
        vec![
            MapActiveInfo {
                map_id: Some(1),
                active_info: vec!["ff".to_string(); 8],
            },
            MapActiveInfo {
                map_id: Some(4),
                active_info: vec!["ff".to_string(); 8],
            },
        ]
    } else {
        map_active_info
    };

    //  2.5 monster_info — should be empty here
    let monster_info = vec![];

    //  2.6 hunting_ground_info — use monsters here instead
    let ground_opt = hunting_rows
        .into_iter()
        .find(|h| req.pack_id.map_or(true, |pid| h.pack_id == Some(pid)));

    let hunting_ground_info = if let Some(h) = ground_opt {
        let monsters = get_monsters_for_hunting_ground(pool, h.index)
            .await
            .unwrap_or_default();

        let hunting_monsters = monsters
            .into_iter()
            .map(|m| MonsterDbInfo {
                monster_id: m.monster_id,
                battle_deck: m.battle_deck,
                active_flag: m.active_flag,
                ..Default::default()
            })
            .collect::<Vec<_>>();

        Some(HuntingGroundDbInfo {
            current_id: h.current_id.or(Some(1)),
            monster_info: hunting_monsters,
            pack_id: h.pack_id.or(Some(1)),
            ..Default::default()
        })
    } else {
        // fallback
        Some(HuntingGroundDbInfo {
            is_auto: Some(false),
            current_id: Some(1),
            highest_id: Some(0),
            monster_info: vec![],
            pack_id: Some(1),
        })
    };

    //  2.7 talent_skill_info — safe to pass empty
    let talent_skill_info = talent_skill_rows
        .into_iter()
        .map(|s| TalentSkillDbInfo {
            group_id: s.group_id,
            end_time: s.end_time,
            cool_time: s.cool_time,
            use_count: s.use_count,
        })
        .collect::<Vec<_>>();

    // 2.8 content_rank_statue_info
    let content_rank_statue_info = statue_rows
        .into_iter()
        .map(|s| ContentRankStatueDbInfo {
            id: s.id,
            season: s.season,
            error_flag: Some(false),
            statue_group_info: vec![],
        })
        .collect::<Vec<_>>();

    // --- 3. clearQuestIds: real cleared-quest history from UserQuest, scoped to the pack being
    // entered when the client tells us which one (same source QuestInfoRequest already uses
    // correctly). This was hardcoded to a permanent `vec![1]` before -- since PackInGameInfo
    // fires on every pack-enter, that meant the client's own "what's actually been cleared"
    // state got stomped back to "just quest 1" every single time, which is what was causing
    // auto-navigate to keep re-targeting an early, already-cleared quest instead of whatever
    // the real current objective is.
    let clear_quest_ids: Vec<i32> = match req.pack_id {
        Some(pack_id) => user_quest_db::get_by_uid_and_pack(pool, uid, pack_id)
            .await
            .unwrap_or_default(),
        None => user_quest_db::get_all_by_uid(pool, uid).await.unwrap_or_default(),
    }
    .into_iter()
    .filter(|r| r.status == 3)
    .map(|r| r.quest_id as i32)
    .collect();

    // --- 4. Build final response ---
    let response = PackInGameInfoResponse {
        char_info,
        quest_info,
        clear_quest_ids,
        position: Some(position),
        talent_npc_info: vec![],
        monster_info,
        talent_object_info: vec![],
        field_buff_info: vec![],
        reputation_info,
        map_active_info,
        research_object_id: vec![],
        hunting_ground_info,
        talent_skill_info,
        content_rank_statue_info,
        statue_reward_obtain_id: vec![],
        reward_info_bundle: Some(RewardDbInfoBundle::default()),
    };

    // --- 5. Encode + wrap into GameResponse ---
    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some(String::new()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::PackInGameInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
