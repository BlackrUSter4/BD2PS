use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CookingResearchRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cooking::cooking_research;
use sqlx::SqlitePool;

#[put("CookingResearch")]
async fn cooking_research_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CookingResearchRequest>("CookingResearch", &body).map_err(|e| {
        tracing::warn!("Failed to parse CookingResearch: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cooking_research::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
