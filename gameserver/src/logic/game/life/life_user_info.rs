use super::{citizen_row_to_dbinfo, helper_row_to_dbinfo, life_user_to_dbinfo};
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeUserInfoRequest, LifeUserInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeUserInfoRequest) -> GameResponse {
    info!("Handling LifeUserInfoRequest: {:?}", req);

    let user = database::db::life::life_user_info::get_or_create(pool, uid)
        .await
        .unwrap_or_default();
    let chunks = database::db::life::life_chunk_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let citizens = database::db::life::life_citizen_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let helpers = database::db::life::life_helper_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();

    let response = LifeUserInfoResponse {
        life_user_info: Some(life_user_to_dbinfo(
            &user,
            chunks.iter().map(|c| c.chunk_id).collect(),
        )),
        life_citizen_list: citizens.iter().map(citizen_row_to_dbinfo).collect(),
        helper_info: helpers.iter().map(helper_row_to_dbinfo).collect(),
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeUserInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
