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
    map::map_active_info::get_map_active_info, quest::quest_info::get_quest_info,
    reputation::reputation_info::get_reputation_info,
    talent::talent_skill_info::get_talent_skill_info,
};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: PackInGameInfoRequest) -> GameResponse {
    info!("Handling PackInGameInfoRequest: {:?}", req);

    // --- 1. Fetch all user data safely ---
    let quest_rows = get_quest_info(pool, uid).await.unwrap_or_default();
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

    // Fetch last saved user position, or fallback to default. Default pack changed to pack21
    // ("Knight of Blood" / Chained Soldier 2 collab) per user request -- this exact MapId/position
    // is a real, live-verified spot inside pack21 (one of its sub-map gate destinations, confirmed
    // reachable this session), not a guess.
    let position = sqlx::query_scalar::<_, Option<String>>(
        "SELECT PackPosition FROM UserPosition WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
    .unwrap_or(None)
    .flatten()
    .unwrap_or_else(|| {
        "{\"MapId\":5,\"PlayerPosition\":{\"x\":-1.0,\"y\":0.0,\"z\":-4.7},\"ColleaguePositions\":null}".to_string()
    });

    // --- 2. Transform DB rows → proto structures ---

    //  2.1 char_info — should be empty at world load
    let char_info = vec![];

    //  2.2 quest_info — must contain at least one entry
    let quest_info = if quest_rows.is_empty() {
        vec![QuestDbInfo {
            id: Some(2),
            value: Some(0),
            object_id: vec![],
            quest_level: Some(0),
            quest_opt: Some(0),
        }]
    } else {
        quest_rows
            .into_iter()
            .map(|q| QuestDbInfo {
                id: q.id,
                value: q.value,
                object_id: vec![],
                quest_level: q.quest_level,
                quest_opt: q.quest_opt,
            })
            .collect()
    };

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

    // --- 3. clearQuestIds should always include [1]
    let clear_quest_ids = vec![1];

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
