use bd2::prost::Message;
use bd2::proto::proto_net::{CharDbInfo, DeckDbInfo, EvilCastleEnterRequest, EvilCastleEnterResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_info;
use database::db::deck::deck_info;
use database::db::evil::evil_castle_info;
use database::models::game::evil::evil_castle_info::EvilCastleInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Entering a tower stage brings your MAIN team deck (not the roguelike
/// mode's own separate deck) — the request carries no deck of its own, so
/// this is the only source available.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleEnterRequest) -> GameResponse {
    info!("Handling EvilCastleEnterRequest: {:?}", req);

    let pack_id = req.pack_id.unwrap_or(0);
    let stage_index = req.stage_index.unwrap_or(1);

    let existing = evil_castle_info::get_by_pack_id(pool, uid, pack_id)
        .await
        .ok()
        .flatten();
    let mut row = existing.unwrap_or(EvilCastleInfo {
        index: 0,
        uid,
        pack_id,
        rank: None,
        stage_index: Some(stage_index),
        retry: Some(0),
        point: Some(0),
        season_highest_point: Some(0),
        is_rewarded: Some(false),
        stage_clear_time: None,
    });
    if req.is_retry == Some(true) {
        row.retry = Some(row.retry.unwrap_or(0) + 1);
    }
    row.stage_index = Some(stage_index);
    row.is_rewarded = Some(false);
    if let Err(e) = evil_castle_info::upsert(pool, &row).await {
        tracing::warn!("EvilCastleEnter: failed to persist progress row: {}", e);
    }

    let char_rows = char_info::get_char_info(pool, uid).await.unwrap_or_default();
    let char_infos: Vec<CharDbInfo> = char_rows
        .into_iter()
        .map(|c| CharDbInfo {
            inven_index: c.inven_index,
            id: c.id,
            hp: c.hp,
            level: c.level,
            costume_id: c.costume_id,
            exp: c.exp,
            use_costume: c.use_costume,
            talent_level: c.talent_level,
            talent_exp: c.talent_exp,
            solidarity_reward: c.solidarity_reward,
            expiry_time: c.expiry_time,
            pictorialbook_info: vec![],
            connect_potential_costume: c.connect_potential_costume,
        })
        .collect();

    let deck_rows = deck_info::get_deck_info(pool, uid).await.unwrap_or_default();
    let deck_infos: Vec<DeckDbInfo> = deck_rows
        .into_iter()
        .map(|d| DeckDbInfo {
            char_inven_index: d.char_inven_index,
            position: d.position,
            sequence: d.sequence,
        })
        .collect();

    let response = EvilCastleEnterResponse {
        monster_info: vec![],
        char_info: char_infos,
        deck_info: deck_infos,
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleEnter.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
