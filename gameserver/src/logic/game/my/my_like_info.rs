use bd2::prost::Message;
use bd2::proto::proto_net::{MyLikeInfoRequest, MyLikeInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::my::my_like_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real most-recent-like read (was silently returning an empty response with no stub markers
/// despite a real, already-scaffolded MyLikeInfo table sitting unused). The response schema
/// is a single target_owner_index, so this returns the most recently liked target.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MyLikeInfoRequest) -> GameResponse {
    info!("Handling MyLikeInfoRequest: {:?}", req);

    let target_owner_index = db::get_my_like_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .max_by_key(|r| r.date.unwrap_or(0))
        .and_then(|r| r.target_owner_index);

    let response = MyLikeInfoResponse { target_owner_index };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::MyLikeInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
