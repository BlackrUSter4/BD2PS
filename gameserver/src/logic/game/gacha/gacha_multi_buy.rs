use bd2::prost::Message;
use bd2::proto::proto_net::{CharDbInfo, GachaMultiBuyRequest, GachaMultiBuyResponse, GachaMultiBuyResult, Notify, RewardDbInfoBundle};
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

/// One draw per requested banner id, same real-currency-free/placeholder-drawn-character
/// approach as the single-buy `gacha_buy` handler (no weighted drop-rate data exists for any
/// banner to draw a "correct" character from).
pub async fn handle(pool: &SqlitePool, uid: i64, req: GachaMultiBuyRequest) -> GameResponse {
    info!("Handling GachaMultiBuyRequest: {:?}", req);

    let game_data = exceldb::get();
    let now = chrono::Utc::now().timestamp_millis();
    let mut results = Vec::with_capacity(req.id.len());

    for (i, id) in req.id.iter().enumerate() {
        let mut char_infos = Vec::new();
        if let Some(picked) = game_data.chartable.all().choose(&mut rand::thread_rng()) {
            let row = CharInfo {
                index: 0,
                uid,
                inven_index: Some(now + i as i64),
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
            if insert_char(pool, &row).await.is_ok() {
                char_infos.push(CharDbInfo {
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
                });
            }
        }

        let log = GachaLogInfo {
            index: 0,
            uid,
            gacha_group_id: Some(*id),
            gacha_id: Some(*id),
            buy_type: None,
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
        let _ = add_gacha_log_info(pool, &log).await;

        results.push(GachaMultiBuyResult {
            id: Some(*id),
            reward_info_bundle: Some(RewardDbInfoBundle {
                char_info: char_infos,
                ..Default::default()
            }),
            get_point: Some(0),
            gacha_fixed_info: vec![],
            selection_apply_sort_id: vec![],
            first_auto_revive_set_char_inven_index: None,
        });
    }

    let response = GachaMultiBuyResponse { results };
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
    let (route, code) = PacketCodeType::GachaMultiBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
