use bd2::prost::Message;
use bd2::proto::proto_net::{
    ItemDbInfo, MyRoomDbInfo, MyRoomPresetDbInfo, MyRoomPresetInfoRequest, MyRoomPresetInfoResponse,
    Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::my::my_room_preset_info;
use sqlx::SqlitePool;
use tracing::info;

const PRESET_TYPE_MY: i32 = 1;
const PRESET_TYPE_USER: i32 = 2;

fn row_to_proto(row: &database::models::game::my::my_room_preset_info::MyRoomPresetInfo) -> MyRoomPresetDbInfo {
    let item_info: Vec<ItemDbInfo> = row
        .item_info_json
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();
    let my_room: Vec<MyRoomDbInfo> = row
        .room_info_json
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();
    MyRoomPresetDbInfo {
        slot: Some(row.slot),
        name: row.name.clone(),
        item_info,
        my_room,
    }
}

pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomPresetInfoRequest) -> GameResponse {
    info!("Handling MyRoomPresetInfoRequest: {:?}", req);

    let my_preset_list = my_room_preset_info::get_all(pool, uid, PRESET_TYPE_MY)
        .await
        .unwrap_or_default()
        .iter()
        .map(row_to_proto)
        .collect();
    let user_preset_list = my_room_preset_info::get_all(pool, uid, PRESET_TYPE_USER)
        .await
        .unwrap_or_default()
        .iter()
        .map(row_to_proto)
        .collect();

    let response = MyRoomPresetInfoResponse {
        my_preset_list,
        user_preset_list,
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomPresetInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
