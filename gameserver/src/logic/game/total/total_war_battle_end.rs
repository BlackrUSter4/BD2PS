use bd2::prost::Message;
use bd2::proto::proto_net::{TotalWarBattleEndRequest, TotalWarBattleEndResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::total::total_war_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::{compute_rank, default_notify, merge_scores, parse_scores};

/// Real score accumulation — the client reports its own battle outcome (same
/// client-simulates/server-trusts pattern as every other battle-adjacent system in this
/// project; no server-side combat resolution exists anywhere here). Scores persist and
/// accumulate for real per category id.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: TotalWarBattleEndRequest,
) -> GameResponse {
    info!("Handling TotalWarBattleEndRequest: {:?}", req);

    let mut row = match db::get_or_create(pool, uid).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("TotalWarBattleEnd get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let mut scores = parse_scores(&row);
    merge_scores(&mut scores, &req.score_info);
    row.score_info_index = serde_json::to_string(&scores).ok();

    if let Err(e) = db::update_total_war_info(pool, &row).await {
        tracing::error!("TotalWarBattleEnd update failed: {}", e);
    }

    let (_, top_percent) = compute_rank(pool, uid).await;

    let response = TotalWarBattleEndResponse {
        score_info: scores,
        top_percent: Some(top_percent),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarBattleEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
