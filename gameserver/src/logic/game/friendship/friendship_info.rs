use bd2::prost::Message;
use bd2::proto::proto_net::{CounselingDbInfo, FriendshipDbInfo, FriendshipInfoRequest, FriendshipInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::friendship::{friendship_counseling_session, friendship_info};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: FriendshipInfoRequest) -> GameResponse {
    info!("Handling FriendshipInfoRequest: {:?}", req);

    let rows = friendship_info::get_all(pool, uid).await;
    let friendship_info: Vec<FriendshipDbInfo> = rows
        .into_iter()
        .map(|r| FriendshipDbInfo {
            friendship_costume_id: Some(r.costume_id),
            level: Some(r.level),
            exp: Some(r.exp),
            last_counseling_date: r.last_counseling_date,
        })
        .collect();

    let sessions = friendship_counseling_session::get_all(pool, uid).await;
    let mut by_costume: std::collections::BTreeMap<i32, Vec<i32>> = std::collections::BTreeMap::new();
    for (costume_id, session_id) in sessions {
        by_costume.entry(costume_id).or_default().push(session_id);
    }
    let counseling_info: Vec<CounselingDbInfo> = by_costume
        .into_iter()
        .map(|(costume_id, session_ids)| CounselingDbInfo {
            friendship_costume_id: Some(costume_id),
            counseling_session_id: session_ids,
        })
        .collect();

    let response = FriendshipInfoResponse {
        friendship_info,
        counseling_info,
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::FriendshipInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
