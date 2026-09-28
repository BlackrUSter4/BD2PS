use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::NoticeDetailInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::notice::notice_detail_info;
use sqlx::SqlitePool;

#[put("NoticeDetailInfo")]
async fn notice_detail_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<NoticeDetailInfoRequest>("NoticeDetailInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse NoticeDetailInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = notice_detail_info::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
