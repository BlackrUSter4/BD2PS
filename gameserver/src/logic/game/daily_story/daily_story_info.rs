use bd2::prost::Message;
use bd2::proto::proto_net::{DailyStoryInfoRequest, DailyStoryInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::daily_story::daily_story_clear_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: DailyStoryInfoRequest) -> GameResponse {
    info!("Handling DailyStoryInfoRequest: {:?}", req);

    let info = daily_story_clear_info::get_cleared(pool, uid).await;

    let response = DailyStoryInfoResponse { info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::DailyStoryInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
