use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingBiteStartCheatRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_bite_start_cheat;
use sqlx::SqlitePool;

#[put("FishingBiteStartCheat")]
async fn fishing_bite_start_cheat_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingBiteStartCheatRequest>("FishingBiteStartCheat", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingBiteStartCheat: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_bite_start_cheat::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
