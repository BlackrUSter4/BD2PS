use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SeasonRewardInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::season::season_reward_info;
use sqlx::SqlitePool;

#[put("SeasonRewardInfo")]
async fn season_reward_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SeasonRewardInfoRequest>("SeasonRewardInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse SeasonRewardInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = season_reward_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
