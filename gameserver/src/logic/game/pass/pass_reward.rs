use bd2::prost::Message;
use bd2::proto::proto_net::{PassRewardRequest, PassRewardResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, pass::{pass_info as pass_db, pass_reward_info as reward_db}};
use sqlx::SqlitePool;
use tracing::info;

/// Real claim-once pass-level reward against PassLevelTable's real basic/premium reward
/// triples, gated by the account's real accumulated exp and (for the premium reward) real
/// PassInfo.active_premium_1 purchase flag.
pub async fn handle(pool: &SqlitePool, uid: i64, req: PassRewardRequest) -> GameResponse {
    info!("Handling PassRewardRequest: {:?}", req);

    let mut item_infos = Vec::new();
    let mut newbie_pass_step = None;

    if let Some(pass_id) = req.pass_id {
        let game_data = data::exceldb::get();
        let pass_row = pass_db::get_by_pass_id(pool, uid, pass_id).await.ok().flatten();
        let current_exp = pass_row.as_ref().and_then(|r| r.exp).unwrap_or(0);
        let has_premium = pass_row.as_ref().and_then(|r| r.active_premium_1).unwrap_or(false);

        if let Some(pass_def) = game_data.passtable.get(pass_id) {
            newbie_pass_step = pass_def.newbie_pass_step;

            let levels: Vec<_> = if req.is_all.unwrap_or(false) {
                game_data.passleveltable.by_group(pass_def.pass_level_group_id).collect()
            } else if let Some(id) = req.id {
                game_data.passleveltable.get(id).into_iter().collect()
            } else {
                vec![]
            };

            for level in levels {
                if current_exp < level.next_need_exp.unwrap_or(0) {
                    continue;
                }
                let claim = reward_db::get_by_pass_and_id(pool, uid, pass_id, level.id).await.ok().flatten();
                let already_basic = claim.as_ref().and_then(|c| c.basic).unwrap_or(false);
                let already_premium = claim.as_ref().and_then(|c| c.premium_1).unwrap_or(false);

                let mut claim_basic = false;
                let mut claim_premium = false;

                if !already_basic {
                    let _ = item_info::grant(pool, uid, level.basic_reward_id, level.basic_reward_type, level.basic_reward_count.max(1)).await;
                    item_infos.push(ItemDbInfo { id: Some(level.basic_reward_id), r#type: Some(level.basic_reward_type), count: Some(level.basic_reward_count.max(1)), ..Default::default() });
                    claim_basic = true;
                }
                if has_premium && !already_premium {
                    if let Some(reward_id) = level.pass1_reward_id {
                        let ty = level.pass1_reward_type.unwrap_or(1);
                        let count = level.pass1_reward_count.unwrap_or(1).max(1);
                        let _ = item_info::grant(pool, uid, reward_id, ty, count).await;
                        item_infos.push(ItemDbInfo { id: Some(reward_id), r#type: Some(ty), count: Some(count), ..Default::default() });
                    }
                    claim_premium = true;
                }

                if claim_basic || claim_premium {
                    let _ = reward_db::mark_claimed(pool, uid, pass_id, level.id, claim_basic, claim_premium).await;
                }
            }
        }
    }

    let response = PassRewardResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
        newbie_pass_step,
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

    let (route, code) = PacketCodeType::PassReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
