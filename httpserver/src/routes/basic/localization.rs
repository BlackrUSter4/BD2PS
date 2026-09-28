use actix_web::{post, HttpResponse, Responder};
use database::models::game::localization::Localization;
use serde_json;

#[post("/api/gpg/price/localization")]
async fn localization_handler() -> impl Responder {
    // Loads static localization JSON at compile time
    let data: Localization = serde_json::from_str(include_str!(
        "../../../../data/starter/localization_info.json"
    ))
    .expect("Failed to parse localization_info.json");

    HttpResponse::Ok().json(data)
}
