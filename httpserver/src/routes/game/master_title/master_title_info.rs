use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MasterTitleInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::master_title::master_title_info;
use sqlx::SqlitePool;

#[put("MasterTitleInfo")]
async fn master_title_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MasterTitleInfoRequest>("MasterTitleInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse MasterTitleInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = master_title_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
