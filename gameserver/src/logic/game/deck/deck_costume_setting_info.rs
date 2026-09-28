use bd2::prost::Message;
use bd2::proto::proto_net::{
    DeckCostumeSettingDbInfo, DeckCostumeSettingInfoRequest, DeckCostumeSettingInfoResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::deck::deck_costume_setting_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-character saved costume-swap sequence, grouped from the one-row-per-seq-element
/// table back into the proto's nested Vec<i64> shape.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: DeckCostumeSettingInfoRequest,
) -> GameResponse {
    info!("Handling DeckCostumeSettingInfoRequest: {:?}", req);

    let rows = db::get_deck_costume_setting_info(pool, uid).await.unwrap_or_default();
    let mut by_char: std::collections::BTreeMap<i64, Vec<i64>> = std::collections::BTreeMap::new();
    for r in rows {
        by_char
            .entry(r.char_inven_index.unwrap_or(0))
            .or_default()
            .push(r.costume_inven_index_seq);
    }

    let costume_setting_info = by_char
        .into_iter()
        .map(|(char_inven_index, costume_inven_index_seq)| DeckCostumeSettingDbInfo {
            char_inven_index: Some(char_inven_index),
            costume_inven_index_seq,
        })
        .collect();

    let response = DeckCostumeSettingInfoResponse { costume_setting_info };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::DeckCostumeSettingInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
