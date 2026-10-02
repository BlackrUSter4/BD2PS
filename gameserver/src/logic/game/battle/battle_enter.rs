use bd2::prost::Message;
use bd2::proto::proto_net::{BattleEnterRequest, BattleEnterResponse, MonsterDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::battle::battle_session;
use database::db::user::user_position::get_current_pack_id;
use database::models::game::battle::battle_session::BattleSession;
use sqlx::SqlitePool;
use tracing::info;

/// Opens (or replaces) the caller's single in-progress battle session. This
/// project has no server-side battle simulation — the client fights the
/// battle itself and reports the outcome via BattleEnd/BattleEndTest — so
/// Enter's real job is just: remember which stage/monster/pack this battle
/// is for (needed later by BattleEnd, which carries none of that context
/// itself), and hand back real monster stats when a FieldMonsterTable entry
/// exists for the requested monster_id.
///
/// `BattleEnterRequest` itself carries no pack id (unlike e.g.
/// FieldObjectResearchRequest) — resolved via the account's own saved
/// position instead (`get_current_pack_id`, see its doc comment). This also
/// finally gives `BattleSession.pack_id` a real value instead of the
/// permanent `None` it had before 2026-10-02.
pub async fn handle(pool: &SqlitePool, uid: i64, req: BattleEnterRequest) -> GameResponse {
    info!("Handling BattleEnterRequest: {:?}", req);

    let pack_id = get_current_pack_id(pool, uid).await;

    let now = chrono::Utc::now().timestamp_millis();
    let session = BattleSession {
        uid,
        battle_index: None,
        group_id: req.group_id,
        monster_id: req.monster_id,
        pack_id: Some(pack_id),
        battle_deck: req.battle_deck,
        battle_mode: req.battle_mode,
        monster_hunt_id: req.monster_hunt_id,
        stage_magic_group_id: req.stage_magic_group_id,
        stage_magic_id: req.stage_magic_id,
        stage_magic_level: req.stage_magic_level,
        random_seed: None,
        created_at: now,
    };
    if let Err(e) = battle_session::upsert(pool, &session).await {
        tracing::warn!("BattleEnter: failed to persist battle session: {}", e);
    }

    let monster_info = req.monster_id.and_then(|monster_id| {
        exceldb::get()
            .fieldmonstertable
            .get_by_pack(pack_id, monster_id)
            .map(|m| MonsterDbInfo {
                monster_id: Some(monster_id),
                battle_deck: m.battle_deck_id.as_ref().and_then(|v| v.first().copied()),
                respawn_time: None,
                life_end_time: None,
                group_id: req.group_id,
                active_flag: Some(true),
            })
    });

    let response = BattleEnterResponse {
        monster_info,
        battle_deck: req.battle_deck,
        event_schedule_info: None,
        buff_stat_info: vec![],
        monster_hunt_user_info: None,
        engine_type: None,
    };

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

    let (route, code) = PacketCodeType::BattleEnter.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
