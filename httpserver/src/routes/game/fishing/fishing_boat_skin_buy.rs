use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingBoatSkinBuyRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_boat_skin_buy;
use sqlx::SqlitePool;

#[put("FishingBoatSkinBuy")]
async fn fishing_boat_skin_buy_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingBoatSkinBuyRequest>("FishingBoatSkinBuy", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingBoatSkinBuy: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_boat_skin_buy::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
