use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeCheatBuildCompleteRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_cheat_build_complete;
use sqlx::SqlitePool;

#[put("LifeCheatBuildComplete")]
async fn life_cheat_build_complete_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeCheatBuildCompleteRequest>("LifeCheatBuildComplete", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse LifeCheatBuildComplete: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = life_cheat_build_complete::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
