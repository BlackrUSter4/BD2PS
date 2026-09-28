use bd2::prost::Message;
use bd2::proto::proto_net::{MiniEventHubInfoRequest, MiniEventHubInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No `MiniEventHub*Table` master/schedule data exists anywhere in this project — an
/// honestly empty hub list, not a fabricated one, same as the already-real
/// `event_exchange_info`/`event_hub_info` handlers when nothing's scheduled.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: MiniEventHubInfoRequest) -> GameResponse {
    info!("Handling MiniEventHubInfoRequest: {:?}", req);

    let response = MiniEventHubInfoResponse {
        mini_event_hub_info: vec![],
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniEventHubInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
