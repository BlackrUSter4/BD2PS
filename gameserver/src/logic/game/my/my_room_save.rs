use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomSaveRequest, MyRoomSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::my::{my_room_info, my_room_item_position_info};
use database::models::game::my::my_room_item_position_info::MyRoomItemPositionInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::{ensure_default_room, DEFAULT_ROOM_ID};

pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomSaveRequest) -> GameResponse {
    info!("Handling MyRoomSaveRequest: {:?}", req);

    ensure_default_room(pool, uid).await;

    if let Some(room) = &req.room_info {
        let room_id = room.id.unwrap_or(DEFAULT_ROOM_ID);

        if my_room_info::get_by_uid_and_id(pool, uid, room_id)
            .await
            .ok()
            .flatten()
            .is_some()
        {
            let _ = my_room_info::update_name_hidden(
                pool,
                uid,
                room_id,
                room.name.as_deref(),
                room.is_hidden.map(|v| v as i32),
            )
            .await;
        }

        let _ = my_room_item_position_info::delete_by_uid_and_room(pool, uid, room_id).await;
        for pos in &room.my_room_position_info {
            let _ = my_room_item_position_info::insert(
                pool,
                &MyRoomItemPositionInfo {
                    index: 0,
                    uid,
                    inven_index: pos.inven_index,
                    object_type: pos.object_type,
                    position_type: pos.position_type,
                    x: pos.x,
                    y: pos.y,
                    rotate: pos.rotate,
                    interact: pos.interact,
                    item_animation: pos.item_animation,
                    is_wall_hidden: pos.is_wall_hidden,
                    room_id,
                },
            )
            .await;
        }
    }

    let response = MyRoomSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
