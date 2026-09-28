use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ServerNowTimeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::server::server_now_time;
use sqlx::SqlitePool;

#[put("ServerNowTime")]
async fn server_now_time_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ServerNowTimeRequest>("ServerNowTime", &body).map_err(|e| {
        tracing::warn!("Failed to parse ServerNowTime: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = server_now_time::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
