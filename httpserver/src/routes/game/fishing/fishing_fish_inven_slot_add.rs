use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingFishInvenSlotAddRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_fish_inven_slot_add;
use sqlx::SqlitePool;

#[put("FishingFishInvenSlotAdd")]
async fn fishing_fish_inven_slot_add_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingFishInvenSlotAddRequest>("FishingFishInvenSlotAdd", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingFishInvenSlotAdd: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_fish_inven_slot_add::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
