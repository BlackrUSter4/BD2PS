use bd2::proto::proto_net::{
    server_info_response::ServerInfo, ServerInfoRequest, ServerInfoResponse,
};

use bd2::prost::Message;
use crypto::network::BaseResponse;
use tracing::info;

pub async fn handle(req: ServerInfoRequest) -> BaseResponse {
    info!("Handling server info request: {:?}", req.seq);

    let server_info = ServerInfo {
        region: Some(0),
        game_server_info: Some("https://api.bd2.pmang.cloud/".to_string()),
        cdn_info: Some("https://bd2-cdn.akamaized.net/ServerData".to_string()),
        open_flag: Some(0),
        log_server_info: Some("https://loki.bd2.pmang.cloud/loki/api/v1/push".to_string()),
        char_server_info: Some("127.0.0.1:38501".to_string()),
        coupon_web_info: Some("https://redeem.bd2.pmang.cloud/bd2/index.html".to_string()),
        game_data_info: Some("https://bd2-cdn.akamaized.net/GameData".to_string()),
        game_data_version: Some("20260923193640".to_string()),
    };

    let response = ServerInfoResponse {
        info_list: vec![server_info],
    };

    let resp_bytes = response.encode_to_vec();

    BaseResponse::success(&resp_bytes)
}
