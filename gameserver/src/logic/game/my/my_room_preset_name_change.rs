use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomPresetNameChangeRequest, MyRoomPresetNameChangeResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::my::my_room_preset_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomPresetNameChangeRequest) -> GameResponse {
    info!("Handling MyRoomPresetNameChangeRequest: {:?}", req);

    if let (Some(slot), Some(name)) = (req.slot, req.name.as_deref()) {
        let preset_type = req.preset_type.unwrap_or(1);
        let _ = my_room_preset_info::update_name(pool, uid, preset_type, slot, name).await;
    }

    let response = MyRoomPresetNameChangeResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomPresetNameChange.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
