use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeCraftingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_crafting;
use sqlx::SqlitePool;

#[put("LifeCrafting")]
async fn life_crafting_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeCraftingRequest>("LifeCrafting", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeCrafting: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_crafting::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
