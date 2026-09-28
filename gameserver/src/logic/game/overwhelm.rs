use bd2::prost::Message;
use bd2::proto::proto_net::{MonsterDbInfo, Notify, OverwhelmRequest, OverwhelmResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::overwhelm::{overwhelm_monster_info as monster_db, overwhelm_quest_update_info as quest_db};
use database::models::game::overwhelm::{
    overwhelm_monster_info::OverwhelmMonsterInfo, overwhelm_quest_update_info::OverwhelmQuestUpdateInfo,
};
use sqlx::SqlitePool;
use tracing::info;

/// Real persistence into the pre-existing (never-exercised) OverwhelmMonsterInfo/
/// OverwhelmQuestUpdateInfo scaffolding: each reported monster engagement is recorded as-is
/// (battle_deck/battle_mode from the client, one row per report — the schema has no unique key
/// to upsert against), and each quest's repeated quest_value array is stored one row per element
/// (same "one row per list element" pattern used elsewhere in this project, e.g. EventHubSettingInfo).
/// No Overwhelm-specific reward/respawn-timer master table was identified within this pass's
/// scope, so reward_bundle/char_info/respawn_time/life_end_time/active_flag stay honestly empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: OverwhelmRequest) -> GameResponse {
    info!("Handling OverwhelmRequest: {:?}", req);

    for m in &req.monster_info {
        let _ = monster_db::add_overwhelm_monster_info(
            pool,
            &OverwhelmMonsterInfo {
                index: 0,
                uid,
                group_id: m.group_id,
                monster_id: m.monster_id,
                battle_deck: m.battle_deck,
                battle_mode: m.battle_mode,
            },
        )
        .await;
    }

    let mut update_quest_id = vec![];
    for q in &req.quest_info {
        if let Some(quest_id) = q.quest_id {
            update_quest_id.push(quest_id);
            for value in &q.quest_value {
                let _ = quest_db::add_overwhelm_quest_update_info(
                    pool,
                    &OverwhelmQuestUpdateInfo { index: 0, uid, quest_id: Some(quest_id), pack_id: q.pack_id, quest_value: *value },
                )
                .await;
            }
        }
    }

    let monster_info: Vec<MonsterDbInfo> = monster_db::get_overwhelm_monster_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| MonsterDbInfo {
            monster_id: r.monster_id,
            battle_deck: r.battle_deck,
            respawn_time: None,
            life_end_time: None,
            group_id: r.group_id,
            active_flag: None,
        })
        .collect();

    let response = OverwhelmResponse { monster_info, update_quest_id, reward_bundle: None, char_info: vec![] };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::Overwhelm.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
