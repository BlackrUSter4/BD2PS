use actix_web::{HttpResponse, Result, put, web};
use bd2::proto::proto_net::CookingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cooking::cooking;
use sqlx::SqlitePool;

#[put("Cooking")]
async fn cooking_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CookingRequest>("Cooking", &body).map_err(|e| {
        tracing::warn!("Failed to parse Cooking: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cooking::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
