use actix_web::{post, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize, Debug)]
struct StateCheckInfoJsonRequest {
    pub seq: i32,
}

#[derive(Serialize, Debug)]
struct StateCheckInfoJsonResponse {
    pub country: String,
    #[serde(rename = "errorType")]
    pub error_type: i32,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

#[post("/StateCheckInfoJson")]
async fn state_check_info_handle(body: String) -> Result<HttpResponse> {
    // Try to parse the raw body string as JSON
    let req: StateCheckInfoJsonRequest = match serde_json::from_str(&body) {
        Ok(r) => {
            info!("Parsed request successfully: {:?}", r);
            r
        }
        Err(e) => {
            info!("Failed to parse request: {}", e);
            let error_response = StateCheckInfoJsonResponse {
                country: "".into(),
                error_type: 400,
                error_message: format!("Invalid JSON: {}", e),
            };
            return Ok(HttpResponse::BadRequest().json(error_response));
        }
    };

    info!("StateCheckInfoJson Request: seq = {}", req.seq);

    let response = StateCheckInfoJsonResponse {
        country: "US".to_string(),
        error_type: 0,
        error_message: "".to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}
