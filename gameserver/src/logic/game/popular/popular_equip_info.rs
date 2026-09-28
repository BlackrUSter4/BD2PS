use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, PopularEquipInfoRequest, PopularEquipInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No reliable master-equip-id/slot-type join path exists from EquipInfo (its real item id
/// and slot type live behind BaseInfoIndex indirection this project never modeled further) to
/// compute a real cross-account popularity ranking — honestly empty rather than fabricated.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: PopularEquipInfoRequest) -> GameResponse {
    info!("Handling PopularEquipInfoRequest: {:?}", req);

    let response = PopularEquipInfoResponse { info: vec![] };

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

    let (route, code) = PacketCodeType::PopularEquipInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
