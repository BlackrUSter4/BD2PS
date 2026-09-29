use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::JoinUserRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::join::join_user;
use sqlx::SqlitePool;

#[put("JoinUser")]
async fn join_user_handler(
    pool: web::Data<SqlitePool>,
    body: String,
) -> Result<HttpResponse> {
    // JoinUser is a public endpoint (auth middleware skips it -- makes sense, there's no account/
    // uid yet at this point), so no uid is ever inserted into request extensions. The handler
    // resolves the real uid itself from the request body's access_token via
    // account::parse_uid_from_token, so its uid param is unused anyway. Don't extract
    // web::ReqData<i64> here, same bug as ServerNowTime/MaintenanceInfo/etc.
    let req = parse_packet::<JoinUserRequest>("JoinUser", &body).map_err(|e| {
        tracing::warn!("Failed to parse JoinUser: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = join_user::handle(&pool, 0, req).await;
    Ok(HttpResponse::Ok().json(response))
}
