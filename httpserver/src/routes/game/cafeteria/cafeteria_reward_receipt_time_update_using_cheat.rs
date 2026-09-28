use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CafeteriaRewardReceiptTimeUpdateUsingCheatRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cafeteria::cafeteria_reward_receipt_time_update_using_cheat;
use sqlx::SqlitePool;

#[put("CafeteriaRewardReceiptTimeUpdateUsingCheat")]
async fn cafeteria_reward_receipt_time_update_using_cheat_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CafeteriaRewardReceiptTimeUpdateUsingCheatRequest>(
        "CafeteriaRewardReceiptTimeUpdateUsingCheat",
        &body,
    )
    .map_err(|e| {
        tracing::warn!(
            "Failed to parse CafeteriaRewardReceiptTimeUpdateUsingCheat: {}",
            e
        );
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cafeteria_reward_receipt_time_update_using_cheat::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
