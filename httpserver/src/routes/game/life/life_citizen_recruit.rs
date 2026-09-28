use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeCitizenRecruitRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_citizen_recruit;
use sqlx::SqlitePool;

#[put("LifeCitizenRecruit")]
async fn life_citizen_recruit_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeCitizenRecruitRequest>("LifeCitizenRecruit", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeCitizenRecruit: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_citizen_recruit::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
