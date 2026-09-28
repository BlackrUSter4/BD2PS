use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeCheatRegenRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_cheat_regen;
use sqlx::SqlitePool;

#[put("LifeCheatRegen")]
async fn life_cheat_regen_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeCheatRegenRequest>("LifeCheatRegen", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeCheatRegen: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_cheat_regen::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
