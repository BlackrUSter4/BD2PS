use bd2::prost::Message;
use bd2::proto::proto_net::{
    CharDbInfo, CharSpecialScoutBuyRequest, CharSpecialScoutBuyResponse, Notify, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::char::{char_info, char_scout_info};
use database::db::costume::costume_info;
use database::db::item::item_info;
use database::models::game::char::char_info::CharInfo;
use database::models::game::costume::costume_info::CostumeInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Grants the featured character for real (inserted into `CharInfo` like a
/// gacha pull), consumes the client-specified items, and marks the featured
/// slot complete. Real end-to-end — the only placeholder is that the actual
/// buy COST isn't independently validated against a table (the client
/// already specifies exactly which items to consume, and no cost table
/// exists to cross-check against, matching the precedent set by CharVote's
/// item-consumption handling).
pub async fn handle(pool: &SqlitePool, uid: i64, req: CharSpecialScoutBuyRequest) -> GameResponse {
    info!("Handling CharSpecialScoutBuyRequest: {:?}", req);

    let Some(id) = req.id else {
        return GameResponse::error(1);
    };

    let game_data = exceldb::get();
    let costume_id = game_data
        .chartable
        .get(id)
        .map(|c| c.default_costume_id)
        .unwrap_or(0);

    let now = chrono::Utc::now().timestamp_millis();
    // See gacha_buy.rs: UseCostume must be a CostumeInfo InvenIndex, not the raw costume design
    // id -- negating the character's own InvenIndex guarantees a collision-free value.
    let costume_inven_index = -now;
    let new_row = CharInfo {
        index: 0,
        uid,
        inven_index: Some(now),
        id: Some(id),
        hp: None,
        level: Some(1),
        costume_id: Some(costume_id),
        exp: Some(0),
        use_costume: Some(costume_inven_index),
        // See gacha_buy.rs: left None, this produced "TalentSkillTable, id:0" and white-box
        // placeholder icons on the Characters screen.
        talent_level: Some(1),
        talent_exp: Some(0),
        solidarity_reward: None,
        expiry_time: None,
        pictorialbook_info_index: None,
        connect_potential_costume: None,
        class_level: 0,
    };
    let _ = char_info::insert(pool, &new_row).await;
    // See gacha_buy.rs: grant the actual costume inventory item UseCostume points to.
    let costume_row = CostumeInfo {
        index: 0,
        uid,
        inven_index: Some(costume_inven_index),
        id: Some(costume_id),
        level: Some(1),
        use_char: Some(now),
        pictorialbook_info_index: None,
        sort_id: None,
        use_my_room_count: None,
        potential_id: None,
        design_id: None,
    };
    let _ = costume_info::insert(pool, &costume_row).await;
    let _ = char_scout_info::mark_complete(pool, uid, id).await;

    for item in &req.item_info {
        if let Some(idx) = item.inven_index {
            let _ = item_info::delete_by_inven_index(pool, uid, idx).await;
        }
    }

    let response = CharSpecialScoutBuyResponse {
        reward_info_bundle: Some(RewardDbInfoBundle {
            char_info: vec![CharDbInfo {
                inven_index: new_row.inven_index,
                id: new_row.id,
                hp: new_row.hp,
                level: new_row.level,
                costume_id: new_row.costume_id,
                exp: new_row.exp,
                use_costume: new_row.use_costume,
                talent_level: new_row.talent_level,
                talent_exp: new_row.talent_exp,
                solidarity_reward: new_row.solidarity_reward,
                expiry_time: new_row.expiry_time,
                connect_potential_costume: new_row.connect_potential_costume,
                pictorialbook_info: vec![],
            }],
            ..Default::default()
        }),
        first_auto_revive_set_char_inven_index: new_row.inven_index,
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
    let (route, code) = PacketCodeType::CharSpecialScoutBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
