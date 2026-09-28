use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PrestigeSkinSetRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::prestige::prestige_skin_set;
use sqlx::SqlitePool;

#[put("PrestigeSkinSet")]
async fn prestige_skin_set_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PrestigeSkinSetRequest>("PrestigeSkinSet", &body).map_err(|e| {
        tracing::warn!("Failed to parse PrestigeSkinSet: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = prestige_skin_set::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
