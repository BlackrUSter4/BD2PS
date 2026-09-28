use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CafeteriaRegularCostumeNoteRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cafeteria::cafeteria_regular_costume_note_reward;
use sqlx::SqlitePool;

#[put("CafeteriaRegularCostumeNoteReward")]
async fn cafeteria_regular_costume_note_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CafeteriaRegularCostumeNoteRewardRequest>(
        "CafeteriaRegularCostumeNoteReward",
        &body,
    )
    .map_err(|e| {
        tracing::warn!("Failed to parse CafeteriaRegularCostumeNoteReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cafeteria_regular_costume_note_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
