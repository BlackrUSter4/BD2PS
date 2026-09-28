use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MyLikeInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::my::my_like_info;
use sqlx::SqlitePool;

#[put("MyLikeInfo")]
async fn my_like_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MyLikeInfoRequest>("MyLikeInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse MyLikeInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = my_like_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
