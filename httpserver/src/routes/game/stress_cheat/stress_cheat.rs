use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::StressCheatRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::stress_cheat::stress_cheat;
use sqlx::SqlitePool;

#[put("StressCheat")]
async fn stress_cheat_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<StressCheatRequest>("StressCheat", &body).map_err(|e| {
        tracing::warn!("Failed to parse StressCheat: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = stress_cheat::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
