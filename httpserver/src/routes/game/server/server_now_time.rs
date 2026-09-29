use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ServerNowTimeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::server::server_now_time;
use sqlx::SqlitePool;

#[put("ServerNowTime")]
async fn server_now_time_handler(
    pool: web::Data<SqlitePool>,
    body: String,
) -> Result<HttpResponse> {
    // ServerNowTime is a public endpoint (auth middleware skips it entirely, so no uid is ever
    // inserted into request extensions) -- the handler doesn't use uid anyway, so don't extract
    // it as web::ReqData<i64> here. That extraction was failing (nothing to extract) and Actix
    // was returning a 500 before this handler body ever ran, on every single call.
    let req = parse_packet::<ServerNowTimeRequest>("ServerNowTime", &body).map_err(|e| {
        tracing::warn!("Failed to parse ServerNowTime: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = server_now_time::handle(&pool, 0, req).await;
    Ok(HttpResponse::Ok().json(response))
}
