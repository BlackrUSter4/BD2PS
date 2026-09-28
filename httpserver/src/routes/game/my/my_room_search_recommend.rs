use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MyRoomSearchRecommendRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::my::my_room_search_recommend;
use sqlx::SqlitePool;

#[put("MyRoomSearchRecommend")]
async fn my_room_search_recommend_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MyRoomSearchRecommendRequest>("MyRoomSearchRecommend", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MyRoomSearchRecommend: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = my_room_search_recommend::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
