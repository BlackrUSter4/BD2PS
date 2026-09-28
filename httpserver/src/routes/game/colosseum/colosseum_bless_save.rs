use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumBlessSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_bless_save;
use sqlx::SqlitePool;

#[put("ColosseumBlessSave")]
async fn colosseum_bless_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumBlessSaveRequest>("ColosseumBlessSave", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumBlessSave: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_bless_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
