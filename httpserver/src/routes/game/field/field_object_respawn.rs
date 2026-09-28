use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FieldObjectRespawnRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::field::field_object_respawn;
use sqlx::SqlitePool;

#[put("FieldObjectRespawn")]
async fn field_object_respawn_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<FieldObjectRespawnRequest>("FieldObjectRespawn", &body).map_err(|e| {
            tracing::warn!("Failed to parse FieldObjectRespawn: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = field_object_respawn::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
