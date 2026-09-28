use bd2::prost::Message;
use bd2::proto::proto_net::{FireWorksInfoRequest, FireWorksInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::fireworks::fireworks_reward_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: FireWorksInfoRequest) -> GameResponse {
    info!("Handling FireWorksInfoRequest: {:?}", req);

    let rewarded_group_id = fireworks_reward_info::get_rewarded_groups(pool, uid).await;

    let response = FireWorksInfoResponse { rewarded_group_id };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::FireWorksInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
