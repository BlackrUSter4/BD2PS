use bd2::prost::Message;
use bd2::proto::proto_net::{BattleEndRequest, BattleEndResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::battle::{battle_char_info, battle_session};
use database::db::item::item_info;
use database::models::game::battle::battle_char_info::BattleCharInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Placeholder reward for a battle win — no table anywhere maps a
/// (group_id/monster_id) pair to a real reward list (unlike QuestClear,
/// which has QuestTable1's reward_* columns to work from), so this grants
/// a small, clearly-placeholder gold amount rather than fabricating a
/// larger "real-looking" reward. Item id 4 / type 1 (gold) matches the
/// convention already established in the SpineInteraction/Friendship
/// rounds for "no reward table exists, grant real gold instead".
const PLACEHOLDER_WIN_GOLD: i32 = 100;

pub async fn handle(pool: &SqlitePool, uid: i64, req: BattleEndRequest) -> GameResponse {
    info!("Handling BattleEndRequest: {:?}", req);

    let won = req.battle_result == Some(1);

    // Persist the caller's post-battle character snapshot for real
    // (replaces any prior rows for this uid — this table is a live
    // per-account roster snapshot, not an append-only log, matching how
    // BattleStart/VerifyState read it back).
    if !req.char_info.is_empty() {
        if let Err(e) = battle_char_info::delete_battle_char_info(pool, uid).await {
            tracing::warn!("BattleEnd: failed to clear old battle_char_info: {}", e);
        }
        for c in &req.char_info {
            let row = BattleCharInfo {
                index: 0,
                uid,
                unique_index: None,
                inven_index: c.inven_index,
                id: c.id,
                hp: c.hp,
                level: c.level,
                costume_inven_index: None,
                costume_id: c.costume_id,
                costume_level: None,
                grid_index: None,
                reserve_costume_id: None,
                is_active_sub_skill_use: None,
                buff_plus_stat: None,
                buff_multiple_stat: None,
                attack_damage: None,
                battle_power: None,
                total_war_play_type: None,
                connect_potential_costume: c.connect_potential_costume,
                key: 0,
                value: 0,
                targeting_count: None,
                supporter_owner_index: None,
                supporter_slot_index: None,
                costume_design_id: None,
            };
            if let Err(e) = battle_char_info::add_battle_char_info(pool, &row).await {
                tracing::warn!("BattleEnd: failed to persist battle_char_info row: {}", e);
            }
        }
    }

    // Consume whatever items the client says got used during the battle
    // (potions, etc.) — real deletion, keyed on the logical InvenIndex the
    // request actually names.
    for inven_index in &req.end_inven_index {
        if let Err(e) = item_info::delete_by_inven_index(pool, uid, *inven_index).await {
            tracing::warn!("BattleEnd: failed to consume item {}: {}", inven_index, e);
        }
    }

    let reward_bundle = if won {
        match item_info::grant(pool, uid, 4, 1, PLACEHOLDER_WIN_GOLD).await {
            Ok(()) => match item_info::find_by_item_id(pool, uid, 4).await {
                Ok(Some(gold)) => Some(RewardDbInfoBundle {
                    item_info: vec![ItemDbInfo {
                        inven_index: gold.inven_index,
                        id: gold.id,
                        r#type: gold.r#type,
                        count: gold.count,
                        keep_flag: gold.keep_flag,
                        time_value: gold.time_value,
                        pictorialbook_info: None,
                        expiry_time: gold.expiry_time,
                        sort_id: gold.sort_id,
                        use_count: gold.use_count,
                    }],
                    ..Default::default()
                }),
                _ => None,
            },
            Err(e) => {
                tracing::warn!("BattleEnd: failed to grant win reward: {}", e);
                None
            }
        }
    } else {
        None
    };

    // This battle is over either way — drop the session so a stray
    // VerifyState/Retry doesn't resurrect a finished battle.
    if let Err(e) = battle_session::delete(pool, uid).await {
        tracing::warn!("BattleEnd: failed to clear battle session: {}", e);
    }

    let response = BattleEndResponse {
        battle_result: req.battle_result,
        char_info: req.char_info.clone(),
        reward_bundle,
        ..Default::default()
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

    let (route, code) = PacketCodeType::BattleEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
