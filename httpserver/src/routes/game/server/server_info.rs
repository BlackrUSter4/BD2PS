use actix_web::{HttpResponse, Result, put, web};
use bd2::proto::proto_net::ServerInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::server::server_info;
use sqlx::SqlitePool;

#[put("ServerInfo")]
async fn server_info_handler(
    _pool: web::Data<SqlitePool>,
    body: String,
) -> Result<HttpResponse> {
    // ServerInfo is a public endpoint (auth middleware skips it, so no uid is ever inserted into
    // request extensions) -- don't extract web::ReqData<i64> here, same bug as ServerNowTime.
    let req = parse_packet::<ServerInfoRequest>("ServerInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ServerInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = server_info::handle(req).await;
    Ok(HttpResponse::Ok().json(response))
}
