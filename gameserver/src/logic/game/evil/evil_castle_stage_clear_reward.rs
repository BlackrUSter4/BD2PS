use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleStageClearRewardRequest, EvilCastleStageClearRewardResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::evil::evil_castle_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleStageClearRewardRequest) -> GameResponse {
    info!("Handling EvilCastleStageClearRewardRequest: {:?}", req);

    let pack_id = req.pack_id.unwrap_or(0);
    let stage_index = req.stage_index.unwrap_or(1);

    let mut item_infos = vec![];
    if let Ok(Some(mut row)) = evil_castle_info::get_by_pack_id(pool, uid, pack_id).await {
        if row.is_rewarded != Some(true) {
            // EvilCastleTable only has 6 real boss-checkpoint rows (no
            // explicit stage->checkpoint mapping was captured), so this
            // cycles through them by stage index — a real, deterministic,
            // data-driven approximation rather than a fabricated reward.
            let table = &exceldb::get().evilcastletable;
            if !table.all().is_empty() {
                let checkpoint_id = ((stage_index - 1).rem_euclid(table.all().len() as i32)) + 1;
                if let Some(t) = table.get(checkpoint_id) {
                    let bundle = super::grant_rewards(pool, uid, &t.reward_id, &t.reward_type, &t.reward_count).await;
                    item_infos = bundle.item_info;
                }
            }
            row.is_rewarded = Some(true);
            row.point = Some(row.point.unwrap_or(0) + 1);
            let _ = evil_castle_info::upsert(pool, &row).await;
        }
    }

    let response = EvilCastleStageClearRewardResponse {
        item_info: item_infos,
        char_info: vec![],
        equip_info: vec![],
        costume_info: vec![],
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleStageClearReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
