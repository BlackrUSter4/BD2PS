use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingBiteFishHpUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_bite_fish_hp_update;
use sqlx::SqlitePool;

#[put("FishingBiteFishHpUpdate")]
async fn fishing_bite_fish_hp_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingBiteFishHpUpdateRequest>("FishingBiteFishHpUpdate", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingBiteFishHpUpdate: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_bite_fish_hp_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
