use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingBiteFishStaminaUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_bite_fish_stamina_update;
use sqlx::SqlitePool;

#[put("FishingBiteFishStaminaUpdate")]
async fn fishing_bite_fish_stamina_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingBiteFishStaminaUpdateRequest>("FishingBiteFishStaminaUpdate", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingBiteFishStaminaUpdate: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_bite_fish_stamina_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
