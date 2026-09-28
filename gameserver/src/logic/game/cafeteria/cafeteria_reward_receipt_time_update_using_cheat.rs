use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaRewardReceiptTimeUpdateUsingCheatRequest,
    CafeteriaRewardReceiptTimeUpdateUsingCheatResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::cafeteria::cafeteria_info;
use sqlx::SqlitePool;
use tracing::info;

/// Dev/testing helper — real, purely mechanical: subtracts real elapsed minutes from the
/// account's own stored `reward_receipt_time` (fast-forwarding cumulative-reward accrual).
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaRewardReceiptTimeUpdateUsingCheatRequest,
) -> GameResponse {
    info!("Handling CafeteriaRewardReceiptTimeUpdateUsingCheatRequest: {:?}", req);

    let mut info_row = match cafeteria_info::get_or_create(pool, uid).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!(
                "CafeteriaRewardReceiptTimeUpdateUsingCheat get_or_create failed: {}",
                e
            );
            return GameResponse::error(1);
        }
    };

    let minutes = req.minute_to_minus.unwrap_or(0) as i64;
    let now_ms = chrono::Utc::now().timestamp_millis();
    let current = info_row.reward_receipt_time.unwrap_or(now_ms);
    info_row.reward_receipt_time = Some(current - minutes * 60_000);

    if let Err(e) = cafeteria_info::update_cafeteria_info(pool, &info_row).await {
        tracing::error!(
            "CafeteriaRewardReceiptTimeUpdateUsingCheat update failed: {}",
            e
        );
        return GameResponse::error(1);
    }

    let response = CafeteriaRewardReceiptTimeUpdateUsingCheatResponse {
        new_reward_receipt_time: info_row.reward_receipt_time,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaRewardReceiptTimeUpdateUsingCheat.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
