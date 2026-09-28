use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeCheatGrowthRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_cheat_growth;
use sqlx::SqlitePool;

#[put("LifeCheatGrowth")]
async fn life_cheat_growth_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeCheatGrowthRequest>("LifeCheatGrowth", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeCheatGrowth: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_cheat_growth::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
