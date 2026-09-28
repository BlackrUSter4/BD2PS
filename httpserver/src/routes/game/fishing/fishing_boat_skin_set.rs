use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingBoatSkinSetRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_boat_skin_set;
use sqlx::SqlitePool;

#[put("FishingBoatSkinSet")]
async fn fishing_boat_skin_set_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingBoatSkinSetRequest>("FishingBoatSkinSet", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingBoatSkinSet: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_boat_skin_set::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
