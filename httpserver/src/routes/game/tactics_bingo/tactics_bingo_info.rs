use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TacticsBingoInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::tactics_bingo::tactics_bingo_info;
use sqlx::SqlitePool;

#[put("TacticsBingoInfo")]
async fn tactics_bingo_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<TacticsBingoInfoRequest>("TacticsBingoInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse TacticsBingoInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = tactics_bingo_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
