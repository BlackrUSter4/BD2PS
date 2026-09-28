use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameSurvivalInfoRequest, MiniGameSurvivalInfoResponse, MiniGameSurvivalStageClearDbInfo,
    MiniGameSurvivalUpgradeDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::{
    mini_game_survival_info as info_db, mini_game_survival_rank_info as rank_db,
    mini_game_survival_stage_clear_info as stage_db, mini_game_survival_upgrade_info as upgrade_db,
};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real account row (seeded starter data for event/active char/map) plus real live rank and
/// progress tables. collection_info stays honestly empty — no per-account collection-unlock
/// table exists in this schema, only the DBInfo proto shape.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameSurvivalInfoRequest) -> GameResponse {
    info!("Handling MiniGameSurvivalInfoRequest: {:?}", req);

    let account_row = info_db::get_mini_game_survival_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .next();

    let event_schedule_id = account_row.as_ref().and_then(|r| r.event_schedule_id);
    let active_char_id = account_row.as_ref().map(|r| vec![r.active_char_id]).unwrap_or_default();
    let active_map_group_id = account_row.as_ref().map(|r| vec![r.active_map_group_id]).unwrap_or_default();

    let user_rank_score = rank_db::get_own(pool, uid).await.ok().flatten().and_then(|r| r.point).map(|p| p as i32);
    let top = rank_db::get_top(pool, 1).await.unwrap_or_default();
    let top_rank_owner_index = top.first().and_then(|r| r.owner_index);
    let top_rank_user_id = top.first().and_then(|r| r.user_id.clone());
    let top_rank_score = top.first().and_then(|r| r.point).map(|p| p as i32);

    let stage_clear_info = stage_db::get_mini_game_survival_stage_clear_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| MiniGameSurvivalStageClearDbInfo { map_group_id: r.map_group_id, map_id: r.map_id })
        .collect();

    let upgrade_info = upgrade_db::get_mini_game_survival_upgrade_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| MiniGameSurvivalUpgradeDbInfo { upgrade_id: r.upgrade_id, level: r.level })
        .collect();

    let response = MiniGameSurvivalInfoResponse {
        event_schedule_id,
        active_char_id,
        active_map_group_id,
        user_rank_score,
        top_rank_owner_index,
        top_rank_user_id,
        top_rank_score,
        stage_clear_info,
        collection_info: vec![],
        upgrade_info,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameSurvivalInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
