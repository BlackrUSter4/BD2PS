use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CashShopPurchaseCountInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cash::cash_shop_purchase_count_info;
use sqlx::SqlitePool;

#[put("CashShopPurchaseCountInfo")]
async fn cash_shop_purchase_count_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CashShopPurchaseCountInfoRequest>("CashShopPurchaseCountInfo", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse CashShopPurchaseCountInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = cash_shop_purchase_count_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
