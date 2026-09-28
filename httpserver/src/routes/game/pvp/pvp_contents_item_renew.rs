use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PvpContentsItemRenewRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pvp::pvp_contents_item_renew;
use sqlx::SqlitePool;

#[put("PvpContentsItemRenew")]
async fn pvp_contents_item_renew_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PvpContentsItemRenewRequest>("PvpContentsItemRenew", &body).map_err(
        |e| {
            tracing::warn!("Failed to parse PvpContentsItemRenew: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        },
    )?;
    let response = pvp_contents_item_renew::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
