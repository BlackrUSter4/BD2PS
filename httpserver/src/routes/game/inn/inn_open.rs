use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::InnOpenRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::inn::inn_open;
use sqlx::SqlitePool;

#[put("InnOpen")]
async fn inn_open_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<InnOpenRequest>("InnOpen", &body).map_err(|e| {
        tracing::warn!("Failed to parse InnOpen: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = inn_open::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
