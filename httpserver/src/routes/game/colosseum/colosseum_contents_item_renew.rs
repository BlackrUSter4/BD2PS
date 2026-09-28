use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumContentsItemRenewRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_contents_item_renew;
use sqlx::SqlitePool;

#[put("ColosseumContentsItemRenew")]
async fn colosseum_contents_item_renew_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumContentsItemRenewRequest>("ColosseumContentsItemRenew", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumContentsItemRenew: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_contents_item_renew::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
