use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingBiteStartRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_bite_start;
use sqlx::SqlitePool;

#[put("FishingBiteStart")]
async fn fishing_bite_start_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingBiteStartRequest>("FishingBiteStart", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingBiteStart: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_bite_start::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
