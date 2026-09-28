use bd2::prost::Message;
use bd2::proto::proto_net::{CharAutoReviveSetRequest, CharAutoReviveSetResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_auto_revive_setting;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CharAutoReviveSetRequest) -> GameResponse {
    info!("Handling CharAutoReviveSetRequest: {:?}", req);

    let can_auto_revive = req.can_char_auto_revive.unwrap_or(false);
    let casting_char_inven_index = req.casting_char_inven_index;
    let _ = char_auto_revive_setting::upsert(pool, uid, can_auto_revive, casting_char_inven_index)
        .await;

    let response = CharAutoReviveSetResponse {
        can_char_auto_revive: Some(can_auto_revive),
        auto_revive_casting_char_inven_index: casting_char_inven_index,
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
    let (route, code) = PacketCodeType::CharAutoReviveSet.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
