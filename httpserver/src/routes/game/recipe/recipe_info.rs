use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::RecipeInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::recipe::recipe_info;
use sqlx::SqlitePool;

#[put("RecipeInfo")]
async fn recipe_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<RecipeInfoRequest>("RecipeInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse RecipeInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = recipe_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
