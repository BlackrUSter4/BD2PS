use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumRankDetailRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_rank_detail;
use sqlx::SqlitePool;

#[put("ColosseumRankDetail")]
async fn colosseum_rank_detail_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumRankDetailRequest>("ColosseumRankDetail", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumRankDetail: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_rank_detail::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
