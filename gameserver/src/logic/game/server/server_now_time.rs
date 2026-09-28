use bd2::prost::Message;
use bd2::proto::proto_net::{ServerNowTimeRequest, ServerNowTimeResponse};

use crypto::network::BaseResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(_pool: &SqlitePool, _uid: i64, req: ServerNowTimeRequest) -> BaseResponse {
    info!("Handling ServerNowTimeRequest: {:?}", req);

    let now_ms = chrono::Utc::now().timestamp_millis();

    let response = ServerNowTimeResponse {
        server_time: Some(now_ms),
    };

    let resp_bytes = response.encode_to_vec();

    BaseResponse::success(&resp_bytes)
}
