use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IdCardRecoveryRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::id::id_card_recovery;
use sqlx::SqlitePool;

#[put("IdCardRecovery")]
async fn id_card_recovery_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IdCardRecoveryRequest>("IdCardRecovery", &body).map_err(|e| {
        tracing::warn!("Failed to parse IdCardRecovery: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = id_card_recovery::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
