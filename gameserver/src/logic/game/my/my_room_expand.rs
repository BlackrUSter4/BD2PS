use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomDbInfo, MyRoomExpandRequest, MyRoomExpandResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::my::my_room_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{ensure_default_room, try_consume_items};

/// Unlocks a new room slot. Real: cost consumption, real MyRoomExpandTable-backed slot
/// creation. `MyRoomExpandTable` (9 real captured tiers) is only used to validate the id is
/// a real tier; no reward is granted by expanding (matches the schema — the response only
/// echoes the new room, no `RewardDBInfoBundle` fields are populated by real client flows
/// for this action as far as the captured schema shows).
pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomExpandRequest) -> GameResponse {
    info!("Handling MyRoomExpandRequest: {:?}", req);

    ensure_default_room(pool, uid).await;

    let mut new_room = None;
    if let Some(id) = req.id {
        let is_real_tier = data::exceldb::get().myroomexpandtable.get(id).is_some();
        let items = req
            .use_item_info
            .as_ref()
            .map(std::slice::from_ref)
            .unwrap_or(&[]);
        let paid = is_real_tier && try_consume_items(pool, uid, items).await;

        if paid {
            let already = my_room_info::get_by_uid_and_id(pool, uid, id)
                .await
                .ok()
                .flatten();
            if already.is_none() {
                let _ = my_room_info::insert(
                    pool,
                    &database::models::game::my::my_room_info::MyRoomInfo {
                        index: 0,
                        uid,
                        id: Some(id),
                        name: Some(format!("Room {id}")),
                        is_hidden: Some(0),
                    },
                )
                .await;
            }
            new_room = Some(MyRoomDbInfo {
                id: Some(id),
                name: Some(format!("Room {id}")),
                my_room_position_info: vec![],
                is_hidden: Some(false),
            });
        }
    }

    let response = MyRoomExpandResponse {
        reward_info_bundle: None,
        my_room: new_room,
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
    let (route, code) = PacketCodeType::MyRoomExpand.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
