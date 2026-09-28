use bd2::prost::Message;
use bd2::proto::proto_net::{CharDbInfo, CharGrowthRequest, CharGrowthResponse, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::char::char_info;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// Same exp-curve placeholder as CharLevelUp (see that file) — Growth uses
/// generic material items instead of "holy water" specifically. No
/// bonus-reward table exists anywhere, so `reward_info_bundle` is
/// documented-empty rather than fabricated.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CharGrowthRequest) -> GameResponse {
    info!("Handling CharGrowthRequest: {:?}", req);

    let Some(inven_index) = req.inven_index else {
        return GameResponse::error(1);
    };

    let Ok(Some(row)) = char_info::get_by_inven_index(pool, uid, inven_index).await else {
        return GameResponse::error(1);
    };

    let game_data = exceldb::get();
    let max_level = game_data
        .chargrowthtable
        .all()
        .iter()
        .map(|g| g.max_level)
        .max()
        .unwrap_or(20);

    let mut level = row.level.unwrap_or(1);
    let mut exp = row.exp.unwrap_or(0);

    let gained: i32 = req.item_info.iter().filter_map(|i| i.count).sum::<i32>() * 15;
    exp += gained;

    while level < max_level {
        let needed = game_data
            .charleveltable
            .all()
            .iter()
            .find(|l| l.id == level)
            .and_then(|l| l.levelup_exp)
            .unwrap_or(100);
        if exp < needed {
            break;
        }
        exp -= needed;
        level += 1;
    }
    if level >= max_level {
        exp = 0;
    }

    let _ = char_info::set_level_exp(pool, uid, inven_index, level, exp).await;
    for item in &req.item_info {
        if let Some(idx) = item.inven_index {
            let _ = item_info::delete_by_inven_index(pool, uid, idx).await;
        }
    }

    let response = CharGrowthResponse {
        char_info: Some(CharDbInfo {
            inven_index: row.inven_index,
            id: row.id,
            hp: row.hp,
            level: Some(level),
            costume_id: row.costume_id,
            exp: Some(exp),
            use_costume: row.use_costume,
            talent_level: row.talent_level,
            talent_exp: row.talent_exp,
            solidarity_reward: row.solidarity_reward,
            expiry_time: row.expiry_time,
            connect_potential_costume: row.connect_potential_costume,
            pictorialbook_info: vec![],
        }),
        reward_info_bundle: Some(RewardDbInfoBundle::default()),
        ..Default::default()
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
    let (route, code) = PacketCodeType::CharGrowth.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
