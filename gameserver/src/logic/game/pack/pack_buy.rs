use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, PackBuyRequest, PackBuyResponse, PackDbInfo, Notify, RewardDbInfoBundle };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pack::pack_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real purchase: marks the account's PackInfo row IsBuy=true and grants PackTable's real
/// buy_reward_id/type/count arrays (79 real packs, most have these configured).
pub async fn handle(pool: &SqlitePool, uid: i64, req: PackBuyRequest) -> GameResponse {
    info!("Handling PackBuyRequest: {:?}", req);

    let mut pack_info = None;
    let mut item_infos = Vec::new();

    if let Some(pack_id) = req.pack_id {
        if let Ok(mut row) = db::get_or_create(pool, uid, pack_id).await {
            row.is_buy = Some(true);
            row.active_time = Some(chrono::Utc::now().timestamp_millis());
            let _ = db::update(pool, &row).await;

            if let Some(pack) = data::exceldb::get().packtable.get(pack_id) {
                if let (Some(ids), Some(types), Some(counts)) =
                    (&pack.buy_reward_id, &pack.buy_reward_type, &pack.buy_reward_count)
                {
                    for i in 0..ids.len() {
                        let id = ids[i];
                        let ty = *types.get(i).unwrap_or(&1);
                        let count = *counts.get(i).unwrap_or(&0);
                        if count > 0 {
                            let _ = database::db::item::item_info::grant(pool, uid, id, ty, count).await;
                            item_infos.push(ItemDbInfo {
                                id: Some(id),
                                r#type: Some(ty),
                                count: Some(count),
                                ..Default::default()
                            });
                        }
                    }
                }
            }

            pack_info = Some(PackDbInfo {
                id: row.id,
                clear_quest_count: row.clear_quest_count,
                is_pack_complete: row.is_pack_complete,
                quest_level: row.quest_level,
                quest_opt: row.quest_opt,
                sub_quest_count: row.sub_quest_count,
                active_time: row.active_time,
                is_buy: row.is_buy,
            });
        }
    }

    let response = PackBuyResponse {
        pack_info,
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
        deck_info: vec![],
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
    
    let (route, code) = PacketCodeType::PackBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}