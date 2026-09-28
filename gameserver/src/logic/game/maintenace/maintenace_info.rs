use bd2::prost::Message;
use bd2::proto::proto_net::{MaintenanceInfo, MaintenanceInfoRequest, MaintenanceInfoResponse};
use crypto::network::BaseResponse;
use tracing::info;

pub async fn handle(req: MaintenanceInfoRequest) -> BaseResponse {
    info!("Maintenance info: {:?}", req.access_token);

    let market_info = MaintenanceInfo {
        market_type: Some(4),
        version: Some("2.8.13".to_string()),
        bundle_version: Some("20260921135230".to_string()),
        is_bundle_update: Some(false),
        maintenance_type: Some(0),
        date: Some("2023-09-18 00-00-00".to_string()),
        region_list: Some("".to_string()),
        use_dsa: Some(1),
        maintenance_url: Some(
            "http://mwe.dq.pmang.com/:brown_Event/maintenance?e=900&market_type=*".to_string(),
        ),
        use_maintenance_url: Some(0),
        download_url: Some("https://www.browndust2.com/".to_string()),
        notice: Some("".to_string()),
        bundle_version_sd: Some("20260921135230".to_string()),
    };

    let response = MaintenanceInfoResponse {
        market_info: Some(market_info),
        ..Default::default()
    };

    BaseResponse::success(&response.encode_to_vec())
}
