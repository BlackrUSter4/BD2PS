use actix_web::{HttpResponse, Result, put, web};
use bd2::proto::proto_net::NoticeInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::notice::notice_info;
use sqlx::SqlitePool;

#[put("NoticeInfo")]
async fn notice_info_handler(
    _pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let _uid = *user_id;
    let req = parse_packet::<NoticeInfoRequest>("NoticeInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse NoticeInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = notice_info::handle(req).await;
    Ok(HttpResponse::Ok().json(response))
}
