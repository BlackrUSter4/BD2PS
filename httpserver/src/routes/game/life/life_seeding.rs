use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeSeedingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_seeding;
use sqlx::SqlitePool;

#[put("LifeSeeding")]
async fn life_seeding_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeSeedingRequest>("LifeSeeding", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeSeeding: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_seeding::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
