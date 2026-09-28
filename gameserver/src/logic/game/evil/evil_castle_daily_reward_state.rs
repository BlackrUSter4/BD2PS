use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleDailyRewardStateRequest, EvilCastleDailyRewardStateResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::evil_castle_daily_reward_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleDailyRewardStateRequest) -> GameResponse {
    info!("Handling EvilCastleDailyRewardStateRequest: {:?}", req);

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let already_claimed = evil_castle_daily_reward_info::get(pool, uid)
        .await
        .ok()
        .flatten()
        .and_then(|r| r.last_claim_date)
        .map(|d| d == today)
        .unwrap_or(false);

    let response = EvilCastleDailyRewardStateResponse {
        is_obtainable_daily_reward: Some(!already_claimed),
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleDailyRewardState.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
