use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumDeckInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_deck_info;
use sqlx::SqlitePool;

#[put("ColosseumDeckInfo")]
async fn colosseum_deck_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumDeckInfoRequest>("ColosseumDeckInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumDeckInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_deck_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
