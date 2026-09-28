use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, TalentNpcDbInfo, TalentSkillDbInfo, TalentSkillUseRequest, TalentSkillUseResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, talent::{talent_npc_info as npc_db, talent_skill_info as skill_db}};
use sqlx::SqlitePool;
use tracing::info;

/// Real current-state echo from TalentSkillInfo/TalentNpcInfo (already-scaffolded tables, just
/// needed gameserver glue) and real food-item consumption. The reward-shaped fields
/// (equip_info/reward_char_info/costume_info/reputation_info/dispatch_info) have no matching
/// per-use grant logic captured anywhere in this project's schema — left honestly empty
/// rather than fabricated.
pub async fn handle(pool: &SqlitePool, uid: i64, req: TalentSkillUseRequest) -> GameResponse {
    info!("Handling TalentSkillUseRequest: {:?}", req);

    if let Some(food_index) = req.food_index {
        let _ = item_info::reduce_by_inven_index(pool, uid, food_index, 1).await;
    }

    let talent_skill_info = skill_db::get_talent_skill_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .next()
        .map(|r| TalentSkillDbInfo { group_id: r.group_id, end_time: r.end_time, cool_time: r.cool_time, use_count: r.use_count });

    let talent_npc_info = npc_db::get_talent_npc_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .next()
        .map(|r| TalentNpcDbInfo { npc_id: r.npc_id, group_id: r.group_id, end_time: r.end_time });

    let response = TalentSkillUseResponse {
        talent_skill_info,
        talent_npc_info,
        item_info: vec![],
        equip_info: vec![],
        reward_char_info: vec![],
        costume_info: vec![],
        reputation_info: vec![],
        dispatch_info: vec![],
        monster_info: vec![],
        add_talent_exp: None,
        is_success: Some(true),
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

    let (route, code) = PacketCodeType::TalentSkillUse.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
