use bd2::prost::Message;
use bd2::proto::proto_net::{FishingFishInvenSlotAddRequest, FishingFishInvenSlotAddResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No payment field on the request, so granted unconditionally (same as FishingMapBuy).
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingFishInvenSlotAddRequest) -> GameResponse {
    info!("Handling FishingFishInvenSlotAddRequest: {:?}", req);

    let add_slot = req.add_slot.unwrap_or(0);
    let slot_count = database::db::fishing::fishing_user_info::add_fish_inven_slot(pool, uid, add_slot)
        .await
        .unwrap_or(20);

    let response = FishingFishInvenSlotAddResponse { slot_count: Some(slot_count) };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingFishInvenSlotAdd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
