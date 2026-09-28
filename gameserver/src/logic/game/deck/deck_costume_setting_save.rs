use bd2::prost::Message;
use bd2::proto::proto_net::{DeckCostumeSettingSaveRequest, DeckCostumeSettingSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::deck::deck_costume_setting_info as db;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: DeckCostumeSettingSaveRequest,
) -> GameResponse {
    info!("Handling DeckCostumeSettingSaveRequest: {:?}", req);

    if let Some(setting) = &req.costume_setting_info {
        if let Some(char_inven_index) = setting.char_inven_index {
            let _ = db::replace_for_char(pool, uid, char_inven_index, &setting.costume_inven_index_seq).await;
        }
    }

    let response = DeckCostumeSettingSaveResponse {};

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

    let (route, code) = PacketCodeType::DeckCostumeSettingSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
