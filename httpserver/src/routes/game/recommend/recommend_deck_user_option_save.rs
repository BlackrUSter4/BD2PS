use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::RecommendDeckUserOptionSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::recommend::recommend_deck_user_option_save;
use sqlx::SqlitePool;

#[put("RecommendDeckUserOptionSave")]
async fn recommend_deck_user_option_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<RecommendDeckUserOptionSaveRequest>("RecommendDeckUserOptionSave", &body)
            .map_err(|e| {
                tracing::warn!("Failed to parse RecommendDeckUserOptionSave: {}", e);
                actix_web::error::ErrorBadRequest("Invalid packet")
            })?;
    let response = recommend_deck_user_option_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
