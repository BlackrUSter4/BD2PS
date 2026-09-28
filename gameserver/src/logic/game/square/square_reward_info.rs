use bd2::prost::Message;
use bd2::proto::proto_net::{SquareRewardInfoRequest, SquareRewardInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::square::square_reward_info as db;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: SquareRewardInfoRequest) -> GameResponse {
    info!("Handling SquareRewardInfoRequest: {:?}", req);

    let row = db::get_or_default(pool, uid).await;
    let today = chrono::Utc::now().date_naive().to_string();
    let is_obtainable_daily_reward = row.last_claim_date.as_deref() != Some(today.as_str());

    let response = SquareRewardInfoResponse {
        is_obtainable_daily_reward: Some(is_obtainable_daily_reward),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::SquareRewardInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
