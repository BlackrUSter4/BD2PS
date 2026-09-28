use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, PackEventBattleDbInfo, PackEventBattleInfoRequest, PackEventBattleInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pack::pack_event_battle_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account battle-challenge progress, filtered by the requested event_uid list
/// when given. Judgment call: `BattleChallengeIndex` is a single column despite the
/// proto's repeated field (pre-existing schema granularity gap) — returned as a
/// one-element list.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: PackEventBattleInfoRequest,
) -> GameResponse {
    info!("Handling PackEventBattleInfoRequest: {:?}", req);

    let rows = db::get_pack_event_battle_info(pool, uid).await.unwrap_or_default();
    let info = rows
        .into_iter()
        .filter(|r| req.event_uid.is_empty() || r.event_uid.map(|e| req.event_uid.contains(&e)).unwrap_or(false))
        .map(|r| PackEventBattleDbInfo {
            event_uid: r.event_uid,
            group_id: r.group_id,
            id: r.id,
            battle_challenge_index: vec![r.battle_challenge_index],
        })
        .collect();

    let response = PackEventBattleInfoResponse { info };

    let resp_bytes = response.encode_to_vec();

    // Notify same as others
    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::PackEventBattleInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
