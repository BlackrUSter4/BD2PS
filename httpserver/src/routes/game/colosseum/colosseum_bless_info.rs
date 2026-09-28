use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumBlessInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_bless_info;
use sqlx::SqlitePool;

#[put("ColosseumBlessInfo")]
async fn colosseum_bless_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumBlessInfoRequest>("ColosseumBlessInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumBlessInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_bless_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
