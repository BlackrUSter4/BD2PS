use bd2::prost::Message;
use bd2::proto::proto_net::{FriendshipSpecialEpisodeDbInfo, FriendshipSpecialEpisodeInfoRequest, FriendshipSpecialEpisodeInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::friendship::friendship_special_episode;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: FriendshipSpecialEpisodeInfoRequest) -> GameResponse {
    info!("Handling FriendshipSpecialEpisodeInfoRequest: {:?}", req);

    let cleared = friendship_special_episode::get_all(pool, uid).await;
    let info: Vec<FriendshipSpecialEpisodeDbInfo> = cleared
        .into_iter()
        .map(|(group_id, id)| FriendshipSpecialEpisodeDbInfo {
            group_id: Some(group_id),
            id: Some(id),
        })
        .collect();

    let response = FriendshipSpecialEpisodeInfoResponse { info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::FriendshipSpecialEpisodeInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
