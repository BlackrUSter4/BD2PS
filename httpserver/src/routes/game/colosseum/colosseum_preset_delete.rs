use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumPresetDeleteRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_preset_delete;
use sqlx::SqlitePool;

#[put("ColosseumPresetDelete")]
async fn colosseum_preset_delete_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumPresetDeleteRequest>("ColosseumPresetDelete", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumPresetDelete: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_preset_delete::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
