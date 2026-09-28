use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingCollectionInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_collection_info;
use sqlx::SqlitePool;

#[put("FishingCollectionInfo")]
async fn fishing_collection_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingCollectionInfoRequest>("FishingCollectionInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingCollectionInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_collection_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
