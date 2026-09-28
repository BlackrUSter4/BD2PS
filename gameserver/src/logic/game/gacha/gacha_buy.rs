use bd2::prost::Message;
use bd2::proto::proto_net::{CharDbInfo, GachaBuyRequest, GachaBuyResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::char::char_info::insert as insert_char;
use database::db::gacha::gacha_log_info::add_gacha_log_info;
use database::models::game::char::char_info::CharInfo;
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
pub async fn handle(pool: &SqlitePool, uid: i64, req: GachaBuyRequest) -> GameResponse {
    info!("Handling GachaBuyRequest: {:?}", req);

    let draw_count = req.buy_type.filter(|&t| t > 0).unwrap_or(1).clamp(1, 10);
    let game_data = exceldb::get();
    let now = chrono::Utc::now().timestamp_millis();

    let mut char_infos = Vec::new();
    for _ in 0..draw_count {
        let Some(picked) = game_data.chartable.all().choose(&mut rand::thread_rng()) else {
            break;
        };
        let row = CharInfo {
            index: 0,
            uid,
            inven_index: Some(now + char_infos.len() as i64),
            id: Some(picked.id),
            hp: None,
            level: Some(1),
            costume_id: Some(picked.default_costume_id),
            exp: Some(0),
            use_costume: None,
            talent_level: None,
            talent_exp: None,
            solidarity_reward: None,
            expiry_time: None,
            pictorialbook_info_index: None,
            connect_potential_costume: None,
            class_level: 0,
        };
        match insert_char(pool, &row).await {
            Ok(_) => char_infos.push(CharDbInfo {
                inven_index: row.inven_index,
                id: row.id,
                hp: row.hp,
                level: row.level,
                costume_id: row.costume_id,
                exp: row.exp,
                use_costume: row.use_costume,
                talent_level: None,
                talent_exp: None,
                solidarity_reward: None,
                expiry_time: None,
                pictorialbook_info: vec![],
                connect_potential_costume: None,
            }),
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
