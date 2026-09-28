use bd2::prost::Message;
use bd2::proto::proto_net::{CharExpiryRequest, CharExpiryResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_info;
use sqlx::SqlitePool;
use tracing::info;

/// Removes a rental/temporary character whose `ExpiryTime` has passed. Real:
/// the character row is actually deleted. `deck_info` is left empty — the
/// "deck" feature is a separate stub cluster not yet fixed in this project,
/// so this doesn't try to remove the expired char from any deck slot.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CharExpiryRequest) -> GameResponse {
    info!("Handling CharExpiryRequest: {:?}", req);

    if let Some(char_inven_index) = req.char_inven_index {
        let _ = char_info::delete_by_inven_index(pool, uid, char_inven_index).await;
    }

    let response = CharExpiryResponse { deck_info: vec![] };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::CharExpiry.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
