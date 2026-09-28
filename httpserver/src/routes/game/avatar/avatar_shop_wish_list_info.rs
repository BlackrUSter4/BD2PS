use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::AvatarShopWishListInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::avatar::avatar_shop_wish_list_info;
use sqlx::SqlitePool;

#[put("AvatarShopWishListInfo")]
async fn avatar_shop_wish_list_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<AvatarShopWishListInfoRequest>("AvatarShopWishListInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse AvatarShopWishListInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = avatar_shop_wish_list_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
