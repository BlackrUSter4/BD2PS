use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharGrowthRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_growth;
use sqlx::SqlitePool;

#[put("CharGrowth")]
async fn char_growth_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharGrowthRequest>("CharGrowth", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharGrowth: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_growth::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
