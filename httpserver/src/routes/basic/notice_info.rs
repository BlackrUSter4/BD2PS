use actix_web::{HttpResponse, Result, put};
use bd2::proto::proto_net::NoticeInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::notice::notice_info;

#[put("NoticeInfo")]
async fn notice_info_handler(body: String) -> Result<HttpResponse> {
    let req = parse_packet::<NoticeInfoRequest>("NoticeInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse NoticeInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = notice_info::handle(req).await;
    Ok(HttpResponse::Ok().json(response))
}
