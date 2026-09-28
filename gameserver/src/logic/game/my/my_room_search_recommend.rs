use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomSearchRecommendRequest, MyRoomSearchRecommendResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::build_user_info;

/// Real other accounts when any exist (random selection, since no recommendation-score
/// data exists to rank by); otherwise an honestly empty list. `MyRoomDefaultTable`'s real
/// `otherPlayerListCount` (10) caps how many are returned.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomSearchRecommendRequest) -> GameResponse {
    info!("Handling MyRoomSearchRecommendRequest: {:?}", req);

    let limit = data::exceldb::get()
        .myroomdefaulttable
        .all()
        .first()
        .map(|r| r.other_player_list_count)
        .unwrap_or(10);

    let rows: Vec<(i64,)> =
        sqlx::query_as("SELECT Uid FROM Account WHERE Uid != ? ORDER BY RANDOM() LIMIT ?")
            .bind(uid)
            .bind(limit)
            .fetch_all(pool)
            .await
            .unwrap_or_default();

    let mut room_info = Vec::new();
    for (other_uid,) in rows {
        room_info.push(build_user_info(pool, other_uid).await);
    }

    let response = MyRoomSearchRecommendResponse { room_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomSearchRecommend.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
