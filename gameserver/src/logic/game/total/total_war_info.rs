use bd2::prost::Message;
use bd2::proto::proto_net::{TotalWarInfoRequest, TotalWarInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::total::total_war_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::{compute_rank, default_notify, parse_scores};

pub async fn handle(pool: &SqlitePool, uid: i64, req: TotalWarInfoRequest) -> GameResponse {
    info!("Handling TotalWarInfoRequest: {:?}", req);

    let row = match db::get_or_create(pool, uid).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("TotalWarInfo get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let (top_ranker_score, top_percent) = compute_rank(pool, uid).await;

    let response = TotalWarInfoResponse {
        score_info: parse_scores(&row),
        top_percent: Some(top_percent),
        top_ranker_score: Some(top_ranker_score),
        engine_type: Some(0), // Define_BattleEngineType::BATTLE_ENGINE_V1 — no per-account choice exists anywhere
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
