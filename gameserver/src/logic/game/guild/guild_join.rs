use bd2::prost::Message;
use bd2::proto::proto_net::{DefineGuildMemberRole, DefineJoinResultType, GuildJoinRequest, GuildJoinResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::{guild_base_info, guild_info, guild_join_application, guild_member_info};
use database::models::game::guild::guild_info::GuildInfo as GuildInfoModel;
use database::models::game::guild::guild_member_info::GuildMemberInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::common::{member_list, now_ms, to_guild_db_info};

const GUILD_TYPE_ANY: i64 = 1;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildJoinRequest) -> GameResponse {
    info!("Handling GuildJoinRequest: {:?}", req);
    let now = now_ms();

    let Some(guild_id) = req.id else {
        return build_response(None, vec![], DefineJoinResultType::GuildJoinFailCooldown);
    };

    let Some(base) = guild_base_info::get_by_guild_id(pool, guild_id).await.ok().flatten() else {
        return build_response(None, vec![], DefineJoinResultType::GuildJoinFailCooldown);
    };
    let Some(canonical_info) = guild_info::get_by_guild_base_index(pool, guild_id).await.ok().flatten() else {
        return build_response(None, vec![], DefineJoinResultType::GuildJoinFailCooldown);
    };

    let join_type = canonical_info
        .join_type
        .as_ref()
        .and_then(|v| v.as_i64())
        .unwrap_or(GUILD_TYPE_ANY);

    if join_type != GUILD_TYPE_ANY {
        // Approval required: record an application instead of joining immediately.
        let _ = guild_join_application::insert(pool, guild_id, uid, &uid.to_string(), now).await;
        let guild_db_info = to_guild_db_info(&canonical_info, &base);
        return build_response(Some(guild_db_info), vec![], DefineJoinResultType::GuildJoinFailCooldown);
    }

    do_join(pool, uid, guild_id, &base, now).await;

    let members = member_list(pool, guild_id).await;
    let guild_db_info = to_guild_db_info(&canonical_info, &base);
    build_response(Some(guild_db_info), members, DefineJoinResultType::GuildJoinSuccess)
}

/// Shared by GuildJoin (auto-type) and GuildAccept (owner approving an application).
pub async fn do_join(pool: &SqlitePool, applicant_uid: i64, guild_id: i64, base: &database::models::game::guild::guild_base_info::GuildBaseInfo, now: i64) {
    let info_row = GuildInfoModel {
        index: 0,
        uid: applicant_uid,
        guild_base_info_index: Some(guild_id),
        join_type: None,
        message: None,
        update_date: Some(now),
        date: Some(now),
        member_count: None,
        delete_remaining_time: None,
        notice_update_date: None,
    };
    if let Err(e) = guild_info::add_guild_info(pool, &info_row).await {
        tracing::warn!("GuildJoin: failed to insert joiner's guild info: {}", e);
    }
    let member_row = GuildMemberInfo {
        index: 0,
        uid: applicant_uid,
        id: Some(guild_id),
        owner_index: Some(applicant_uid),
        user_id: Some(applicant_uid.to_string()),
        title_id: None,
        portrait_costume_id: None,
        portrait_costume_design_id: None,
        role: Some(serde_json::json!(DefineGuildMemberRole::GuildRoleMember as i32)),
        point: Some(0),
        supporter_info_index: None,
        last_login_date: Some(now),
        update_date: Some(now),
        date: Some(now),
    };
    if let Err(e) = guild_member_info::add_guild_member_info(pool, &member_row).await {
        tracing::warn!("GuildJoin: failed to insert joiner's member row: {}", e);
    }
    let _ = guild_info::update_member_count(pool, guild_id, 1).await;
    let _ = base;
}

fn build_response(
    guild_info: Option<bd2::proto::proto_net::GuildDbInfo>,
    member_info: Vec<bd2::proto::proto_net::GuildMemberDbInfo>,
    result_type: DefineJoinResultType,
) -> GameResponse {
    let response = GuildJoinResponse {
        guild_info,
        member_info,
        ban_time: None,
        result_type: Some(result_type as i32),
        top_percent: Some(0.0),
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
    let (route, code) = PacketCodeType::GuildJoin.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
