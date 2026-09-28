use bd2::prost::Message;
use bd2::proto::proto_net::{CharAwakeActiveRequest, CharAwakeActiveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::{char_awake_info, char_info};
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CharAwakeActiveRequest) -> GameResponse {
    info!("Handling CharAwakeActiveRequest: {:?}", req);

    let Some(char_inven_index) = req.char_inven_index else {
        return GameResponse::error(1);
    };
    let Ok(Some(char_row)) = char_info::get_by_inven_index(pool, uid, char_inven_index).await
    else {
        return GameResponse::error(1);
    };
    let Some(unique_char_id) = char_row.id else {
        return GameResponse::error(1);
    };

    let _ = char_awake_info::set_awake(pool, uid, unique_char_id).await;
    for item in &req.item_info {
        if let Some(idx) = item.inven_index {
            let _ = item_info::delete_by_inven_index(pool, uid, idx).await;
        }
    }

    let response = CharAwakeActiveResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::CharAwakeActive.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
