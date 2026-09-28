use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomPresetSaveRequest, MyRoomPresetSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::my::my_room_preset_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomPresetSaveRequest) -> GameResponse {
    info!("Handling MyRoomPresetSaveRequest: {:?}", req);

    if let Some(preset) = &req.preset_info {
        if let Some(slot) = preset.slot {
            // No explicit preset_type field on this request: inferred from whether
            // `source_owner_index` points at someone else's room (a "USER" preset — a
            // saved copy of another account's layout) or is absent/self (a "MY" preset).
            let preset_type = match req.source_owner_index {
                Some(src) if src != uid => 2,
                _ => 1,
            };
            let item_json = serde_json::to_string(&preset.item_info).unwrap_or_default();
            let room_json = serde_json::to_string(&preset.my_room).unwrap_or_default();
            let _ = my_room_preset_info::upsert(
                pool,
                uid,
                preset_type,
                slot,
                preset.name.as_deref(),
                req.source_owner_index,
                &item_json,
                &room_json,
            )
            .await;
        }
    }

    let response = MyRoomPresetSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomPresetSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
