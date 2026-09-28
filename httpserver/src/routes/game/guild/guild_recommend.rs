use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildRecommendRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_recommend;
use sqlx::SqlitePool;

#[put("GuildRecommend")]
async fn guild_recommend_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildRecommendRequest>("GuildRecommend", &body).map_err(|e| {
        tracing::warn!("Failed to parse GuildRecommend: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = guild_recommend::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
