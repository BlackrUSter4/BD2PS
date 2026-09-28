use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SelectPlatformOtherDataRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::select::select_platform_other_data;
use sqlx::SqlitePool;

#[put("SelectPlatformOtherData")]
async fn select_platform_other_data_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SelectPlatformOtherDataRequest>("SelectPlatformOtherData", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse SelectPlatformOtherData: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = select_platform_other_data::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
