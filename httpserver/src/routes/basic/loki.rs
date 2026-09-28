use actix_web::{post, HttpResponse, Result};

#[post("loki/api/v1/push")]
async fn loki_handler() -> Result<HttpResponse> {
    let response = String::new();

    Ok(HttpResponse::Ok().json(response))
}
