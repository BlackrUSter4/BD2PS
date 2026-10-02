use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, TalentNpcDbInfo, TalentSkillDbInfo, TalentSkillUseRequest, TalentSkillUseResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::{char::char_info, item::item_info, talent::talent_npc_info as npc_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real talent_skill_info computed fresh from the using character's own TalentTable/
/// TalentSkillTable row (group_id + end_time = now + TalentSkillTable.value_list[1] seconds),
/// cross-checked against the reference server's `GameTalentServer.TalentSkillUse` — which
/// builds this the exact same way and does NOT persist a TalentSkillInfo row at all (the
/// earlier version's DB-echo read was never backed by any real writer). `add_talent_exp` is
/// real TalentLevel/TalentExp progression for `req.inven_index` via `talent::add_talent_exp`,
/// matching the reference's own `charInfoDao.TalentExp += talentSkill.GetExp` exactly. Real
/// food-item consumption. The reward-shaped fields (equip_info/reward_char_info/costume_info/
/// reputation_info/dispatch_info) have no matching per-use grant logic in the reference either
/// — left honestly empty rather than fabricated.
pub async fn handle(pool: &SqlitePool, uid: i64, req: TalentSkillUseRequest) -> GameResponse {
    info!("Handling TalentSkillUseRequest: {:?}", req);

    if let Some(food_index) = req.food_index {
        let _ = item_info::reduce_by_inven_index(pool, uid, food_index, 1).await;
    }

    let mut talent_skill_info = None;
    let mut add_talent_exp = None;

    if let Some(inven_index) = req.inven_index {
        if let Ok(Some(char_row)) = char_info::get_by_inven_index(pool, uid, inven_index).await {
            if let Some(char_id) = char_row.id {
                let db = exceldb::get();
                if let Some(talent) = db
                    .chartable
                    .get(char_id)
                    .and_then(|c| c.talent_id)
                    .and_then(|tid| db.talenttable.get(tid))
                {
                    let level = char_row.talent_level.unwrap_or(0).max(1);
                    if let Some(skill) = db
                        .talentskilltable
                        .by_group(talent.talent_skill_group_id)
                        .find(|r| r.id == level)
                    {
                        let duration = *skill.value_list.get(1).unwrap_or(&0.0) as i64;
                        talent_skill_info = Some(TalentSkillDbInfo {
                            group_id: Some(skill.group_id),
                            end_time: Some(chrono::Utc::now().timestamp_millis() + duration * 1000),
                            cool_time: None,
                            use_count: None,
                        });
                    }
                }
            }
        }
        add_talent_exp = Some(super::add_talent_exp(pool, uid, inven_index, 1).await);
    }

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
        add_talent_exp,
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
