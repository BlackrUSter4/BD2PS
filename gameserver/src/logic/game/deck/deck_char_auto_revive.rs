use bd2::prost::Message;
use bd2::proto::proto_net::{DeckCharAutoReviveRequest, DeckCharAutoReviveResponse, DeckDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::deck::deck_info as deck_db;
use sqlx::SqlitePool;
use tracing::info;

/// No death/HP-zero state is modeled for any character anywhere in this project (confirmed
/// during the Char cluster round — no MaxHp stat exists), so there is nothing to actually
/// revive here. Real part: echoes the account's current real deck back; revive-specific
/// fields (revived chars, talent exp, catalyst consumption) are honestly empty/None rather
/// than fabricated, since no revive event genuinely occurred.
pub async fn handle(pool: &SqlitePool, uid: i64, req: DeckCharAutoReviveRequest) -> GameResponse {
    info!("Handling DeckCharAutoReviveRequest: {:?}", req);

    let deck_info = deck_db::get_deck_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| DeckDbInfo {
            char_inven_index: d.char_inven_index,
            position: d.position,
            sequence: d.sequence,
        })
        .collect();

    let response = DeckCharAutoReviveResponse {
        casting_char_inven_index: req.casting_char_inven_index,
        deck_info,
        field_deck_info: vec![],
        revived_char_inven_index: vec![],
        auto_revive_char_type: None,
        revive_char_info: vec![],
        add_talent_exp: None,
        catalyst: None,
        auto_revive_disabled_type: None,
    };

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

    let (route, code) = PacketCodeType::DeckCharAutoRevive.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
