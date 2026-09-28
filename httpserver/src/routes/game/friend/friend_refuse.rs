use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendRefuseRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friend::friend_refuse;
use sqlx::SqlitePool;

#[put("FriendRefuse")]
async fn friend_refuse_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FriendRefuseRequest>("FriendRefuse", &body).map_err(|e| {
        tracing::warn!("Failed to parse FriendRefuse: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = friend_refuse::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
