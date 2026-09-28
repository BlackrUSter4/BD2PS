use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::RootSortIdInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::root::root_sort_id_info;
use sqlx::SqlitePool;

#[put("RootSortIdInfo")]
async fn root_sort_id_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<RootSortIdInfoRequest>("RootSortIdInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse RootSortIdInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = root_sort_id_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
