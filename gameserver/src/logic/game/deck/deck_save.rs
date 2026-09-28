use bd2::prost::Message;
use bd2::proto::proto_net::{CharDbInfo, DeckDbInfo, DeckSaveRequest, DeckSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{char::char_info, deck::deck_info as deck_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real deck replacement: the account's whole DeckInfo table is replaced with the requested
/// formation. No auto-revive resolution exists anywhere in this project (no MaxHp stat is
/// modeled for any character), so `forced_revive_char_inven_index` is honestly always None.
pub async fn handle(pool: &SqlitePool, uid: i64, req: DeckSaveRequest) -> GameResponse {
    info!("Handling DeckSaveRequest: {:?}", req);

    let entries: Vec<(i64, Option<i32>, Option<i32>)> = req
        .deck_info
        .iter()
        .filter_map(|d| d.char_inven_index.map(|c| (c, d.position, d.sequence)))
        .collect();
    let _ = deck_db::replace_deck_info(pool, uid, &entries).await;

    let first = req.deck_info.first();
    let deck_info = first.map(|d| DeckDbInfo {
        char_inven_index: d.char_inven_index,
        position: d.position,
        sequence: d.sequence,
    });

    let mut char_info_resp = None;
    if let Some(char_inven_index) = first.and_then(|d| d.char_inven_index) {
        if let Ok(Some(row)) = char_info::get_by_inven_index(pool, uid, char_inven_index).await {
            char_info_resp = Some(CharDbInfo {
                inven_index: row.inven_index,
                id: row.id,
                hp: row.hp,
                level: row.level,
                costume_id: row.costume_id,
                exp: row.exp,
                use_costume: row.use_costume,
                talent_level: row.talent_level,
                talent_exp: row.talent_exp,
                solidarity_reward: row.solidarity_reward,
                expiry_time: row.expiry_time,
                pictorialbook_info: vec![],
                connect_potential_costume: row.connect_potential_costume,
            });
        }
    }

    let response = DeckSaveResponse {
        deck_info,
        char_info: char_info_resp,
        forced_revive_char_inven_index: None,
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

    let (route, code) = PacketCodeType::DeckSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
