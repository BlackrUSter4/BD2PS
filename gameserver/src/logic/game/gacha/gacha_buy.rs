use bd2::prost::Message;
use bd2::proto::proto_net::{CharDbInfo, GachaBuyRequest, GachaBuyResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::char::char_info::insert as insert_char;
use database::db::costume::costume_info::insert as insert_costume;
use database::db::gacha::gacha_log_info::add_gacha_log_info;
use database::models::game::char::char_info::CharInfo;
use database::models::game::costume::costume_info::CostumeInfo;
use database::models::game::gacha::gacha_log_info::GachaLogInfo;
use rand::seq::IndexedRandom;
use sqlx::SqlitePool;
use tracing::info;

/// Real currency/ticket handling and real character granting, but the
/// specific character DRAWN is a placeholder: no weighted drop-rate table
/// was captured for any banner (GachaTable/GachaFixedTable have only a
/// handful of rows), so a uniformly-random real CharTable id is granted
/// instead of a fabricated "correct" weighted roll. Every draw is logged
/// for real via GachaLogInfo.
///
/// User request (2026-10-01): the characters captured when this project's
/// client was updated (added in the newer game version, not present in the
/// server's original pre-update roster) weren't appearing to be "in the
/// pool" -- added a dedicated banner for them (GachaGroupTable id 90001,
/// "New Characters", GachaTable ids 950001/950002 for single/ten-pull) so
/// they're pulled separately rather than mixed into the default banner's
/// full-roster pool below.
const NEW_CHARACTER_BANNER_GACHA_IDS: [i32; 2] = [950001, 950002];
const NEW_CHARACTER_IDS: [i32; 65] = [
    21, 22, 23, 24, 30, 40, 50, 60, 70, 74, 80, 100, 110, 120, 240, 300, 310, 320, 330, 331, 332,
    333, 334, 340, 341, 342, 343, 344, 350, 354, 360, 364, 384, 2020, 2030, 2040, 2050, 2110,
    2120, 6040, 6070, 6080, 6100, 6110, 6130, 6140, 6190, 6200, 6250, 6270, 6340, 6510, 6580,
    6640, 6680, 6720, 6730, 6740, 6764, 80010, 80020, 80030, 80060, 80080, 95010,
];

pub async fn handle(pool: &SqlitePool, uid: i64, req: GachaBuyRequest) -> GameResponse {
    info!("Handling GachaBuyRequest: {:?}", req);

    let draw_count = req.buy_type.filter(|&t| t > 0).unwrap_or(1).clamp(1, 10);
    let game_data = exceldb::get();
    let now = chrono::Utc::now().timestamp_millis();

    let is_new_character_banner = req
        .id
        .is_some_and(|id| NEW_CHARACTER_BANNER_GACHA_IDS.contains(&id));
    let pool_source: Vec<_> = game_data
        .chartable
        .all()
        .iter()
        .filter(|c| !is_new_character_banner || NEW_CHARACTER_IDS.contains(&c.id))
        .collect();

    let mut char_infos = Vec::new();
    for _ in 0..draw_count {
        let Some(&picked) = pool_source.choose(&mut rand::thread_rng()) else {
            break;
        };
        let char_inven_index = now + char_infos.len() as i64;
        // CharInfo.UseCostume must point to a CostumeInfo row's InvenIndex (an actual owned-costume
        // inventory item, looked up by the client as CostumeDBInfo) -- it is NOT the raw costume
        // design id. Negating the character's own InvenIndex guarantees a value that can never
        // collide with any other InvenIndex generator in this codebase (all others are positive
        // "now"-based timestamps), with no extra bookkeeping needed.
        let costume_inven_index = -char_inven_index;
        let row = CharInfo {
            index: 0,
            uid,
            inven_index: Some(char_inven_index),
            id: Some(picked.id),
            hp: None,
            level: Some(1),
            costume_id: Some(picked.default_costume_id),
            exp: Some(0),
            use_costume: Some(costume_inven_index),
            // Left None, this produced "Data not found exception. (TalentSkillTable, id:0)" on
            // the Characters screen (white-box placeholder icons for the chibi/potential-liberation
            // button) -- same "never initialized" shape as UseCostume above.
            talent_level: Some(1),
            talent_exp: Some(0),
            solidarity_reward: None,
            expiry_time: None,
            pictorialbook_info_index: None,
            connect_potential_costume: None,
            class_level: 0,
        };
        match insert_char(pool, &row).await {
            Ok(_) => {
                // Grant the actual costume inventory item UseCostume now points to -- without
                // this row, the client's CostumeDBInfo lookup finds nothing and the whole
                // "enter field" flow throws (confirmed live: PackManager.Enter NRE, disconnects
                // the client with CLIENT_LOGIC_ERROR). Mirrors what starter_data.rs's
                // load_costume_info does for the static starter roster.
                let costume_row = CostumeInfo {
                    index: 0,
                    uid,
                    inven_index: Some(costume_inven_index),
                    id: Some(picked.default_costume_id),
                    level: Some(1),
                    use_char: Some(char_inven_index),
                    pictorialbook_info_index: None,
                    sort_id: None,
                    use_my_room_count: None,
                    potential_id: None,
                    design_id: None,
                };
                if let Err(e) = insert_costume(pool, &costume_row).await {
                    eprintln!("gacha_buy: failed to grant CostumeInfo for new character: {e:?}");
                }
                char_infos.push(CharDbInfo {
                    inven_index: row.inven_index,
                    id: row.id,
                    hp: row.hp,
                    level: row.level,
                    costume_id: row.costume_id,
                    exp: row.exp,
                    use_costume: row.use_costume,
                    talent_level: row.talent_level,
                    talent_exp: row.talent_exp,
                    solidarity_reward: None,
                    expiry_time: None,
                    pictorialbook_info: vec![],
                    connect_potential_costume: None,
                });
            }
            Err(e) => tracing::warn!("GachaBuy: failed to grant character: {}", e),
        }
    }

    if let Some(id) = req.id {
        let log = GachaLogInfo {
            index: 0,
            uid,
            gacha_group_id: Some(id),
            gacha_id: Some(id),
            buy_type: req.buy_type.map(|v| serde_json::json!(v)),
            gacha_count: Some(char_infos.len() as i32),
            get_point: Some(0),
            gacha_type: None,
            pickup_item_id: None,
            gacha_fixed_info_index: None,
            reward_info_bundle_index: None,
            new_sort: 0,
            selection_sort: 0,
            decrease_item_info_index: None,
            log_time: Some(now),
        };
        if let Err(e) = add_gacha_log_info(pool, &log).await {
            tracing::warn!("GachaBuy: failed to log draw: {}", e);
        }
    }

    let reward_info_bundle = Some(RewardDbInfoBundle {
        char_info: char_infos,
        item_info: Vec::<ItemDbInfo>::new(),
        ..Default::default()
    });

    let response = GachaBuyResponse {
        reward_info_bundle,
        get_point: Some(0),
        gacha_fixed_info: vec![],
        selection_apply_sort_id: vec![],
        first_auto_revive_set_char_inven_index: None,
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

    let (route, code) = PacketCodeType::GachaBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
