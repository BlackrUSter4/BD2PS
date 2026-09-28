use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameActionUserRecordDbInfo, MiniGameActionUserRecordInfoRequest,
    MiniGameActionUserRecordInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::{mini_game_action_multi_rank_info, mini_game_action_single_rank_info};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real query against this account's own Action-minigame rank rows. In practice these are
/// always empty right now: every MiniGame{Action,Rhythm,Sichuan,Survival}* gameplay handler
/// (mini_game_action_end.rs etc.) is an unfilled TODO stub that never writes to these tables
/// (verified: `grep -c TODO gameserver/src/logic/game/mini/*.rs` shows all 44 pre-existing
/// handlers are stubs, not the "already implemented" state assumed in round 1's categorization).
/// This query will start returning real data automatically once a future round fills those in.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MiniGameActionUserRecordInfoRequest,
) -> GameResponse {
    info!("Handling MiniGameActionUserRecordInfoRequest: {:?}", req);

    let mut record_info: Vec<MiniGameActionUserRecordDbInfo> =
        mini_game_action_single_rank_info::get_mini_game_action_single_rank_info(pool, uid)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|r| MiniGameActionUserRecordDbInfo {
                action_monster: req.stage_type,
                score: r.score,
                top_percent: None,
                user_info: vec![],
            })
            .collect();

    record_info.extend(
        mini_game_action_multi_rank_info::get_mini_game_action_multi_rank_info(pool, uid)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|r| MiniGameActionUserRecordDbInfo {
                action_monster: req.stage_type,
                score: r.score,
                top_percent: None,
                user_info: vec![],
            }),
    );

    let response = MiniGameActionUserRecordInfoResponse { record_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameActionUserRecordInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
