use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomMoveRequest, MyRoomMoveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::my::my_room_info;
use sqlx::SqlitePool;
use tracing::info;

/// Reassigns a room's slot number, swapping with whichever room already holds the target
/// slot if one exists (no separate proto field exists to disambiguate "swap" from "move
/// into empty slot", so both are handled by the same real update).
pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomMoveRequest) -> GameResponse {
    info!("Handling MyRoomMoveRequest: {:?}", req);

    if let (Some(id), Some(target_id)) = (req.id, req.target_id) {
        let occupant = my_room_info::get_by_uid_and_id(pool, uid, target_id)
            .await
            .ok()
            .flatten();

        if let Some(occupant) = occupant {
            // Slot occupied: swap the two rooms' Id values via a temporary sentinel to
            // avoid violating (Uid, Id) expectations mid-update.
            let _ = sqlx::query("UPDATE MyRoomInfo SET Id = -1 WHERE Uid = ? AND Id = ?")
                .bind(uid)
                .bind(id)
                .execute(pool)
                .await;
            let _ = sqlx::query("UPDATE MyRoomInfo SET Id = ? WHERE Uid = ? AND Id = ?")
                .bind(id)
                .bind(uid)
                .bind(target_id)
                .execute(pool)
                .await;
            let _ = sqlx::query("UPDATE MyRoomInfo SET Id = ? WHERE Uid = ? AND Id = -1")
                .bind(target_id)
                .bind(uid)
                .execute(pool)
                .await;
            let _ = occupant;
        } else {
            let _ = sqlx::query("UPDATE MyRoomInfo SET Id = ? WHERE Uid = ? AND Id = ?")
                .bind(target_id)
                .bind(uid)
                .bind(id)
                .execute(pool)
                .await;
        }
    }

    let response = MyRoomMoveResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomMove.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
