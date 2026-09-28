use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CafeteriaRegularCostumeNoteAllRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cafeteria::cafeteria_regular_costume_note_all_reward;
use sqlx::SqlitePool;

#[put("CafeteriaRegularCostumeNoteAllReward")]
async fn cafeteria_regular_costume_note_all_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CafeteriaRegularCostumeNoteAllRewardRequest>(
        "CafeteriaRegularCostumeNoteAllReward",
        &body,
    )
    .map_err(|e| {
        tracing::warn!(
            "Failed to parse CafeteriaRegularCostumeNoteAllReward: {}",
            e
        );
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cafeteria_regular_costume_note_all_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
