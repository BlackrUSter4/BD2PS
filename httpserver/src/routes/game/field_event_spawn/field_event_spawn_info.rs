use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FieldEventSpawnInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::field_event_spawn::field_event_spawn_info;
use sqlx::SqlitePool;

#[put("FieldEventSpawnInfo")]
async fn field_event_spawn_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FieldEventSpawnInfoRequest>("FieldEventSpawnInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse FieldEventSpawnInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = field_event_spawn_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
