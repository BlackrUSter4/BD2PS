use bd2::prost::Message;
use bd2::proto::proto_net::{DefineGuildMemberRole, GuildCreateRequest, GuildCreateResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::{guild_base_info, guild_info, guild_member_info};
use database::models::game::guild::guild_base_info::GuildBaseInfo;
use database::models::game::guild::guild_info::GuildInfo;
use database::models::game::guild::guild_member_info::GuildMemberInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::common::{now_ms, to_guild_db_info};

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildCreateRequest) -> GameResponse {
    info!("Handling GuildCreateRequest: {:?}", req);
    let now = now_ms();

    let base = GuildBaseInfo {
        index: 0,
        uid,
        id: None,
        name: req.name.clone(),
        icon: req.icon,
        icon_color: req.icon_color.clone(),
        grade: Some(1),
    };
    let base_index = match guild_base_info::insert(pool, &base).await {
        Ok(i) => i,
        Err(e) => {
            tracing::warn!("GuildCreate: failed to insert base info: {}", e);
            return GameResponse::error(1);
        }
    };
    // Guild's public id doubles as its own row's global rowid.
    let _ = sqlx::query("UPDATE GuildBaseInfo SET Id = ? WHERE \"Index\" = ?")
        .bind(base_index)
        .bind(base_index)
        .execute(pool)
        .await;
    let base = GuildBaseInfo {
        index: base_index,
        id: Some(base_index),
        ..base
    };

    let info_row = GuildInfo {
        index: 0,
        uid,
        guild_base_info_index: Some(base_index),
        join_type: req.join_type.map(|v| serde_json::json!(v)),
        message: req.message.clone(),
        update_date: Some(now),
        date: Some(now),
        member_count: Some(1),
        delete_remaining_time: None,
        notice_update_date: None,
    };
    if let Err(e) = guild_info::add_guild_info(pool, &info_row).await {
        tracing::warn!("GuildCreate: failed to insert guild info: {}", e);
        return GameResponse::error(1);
    }

    let member_row = GuildMemberInfo {
        index: 0,
        uid,
        id: Some(base_index),
        owner_index: Some(uid),
        user_id: Some(uid.to_string()),
        title_id: None,
        portrait_costume_id: None,
        portrait_costume_design_id: None,
        role: Some(serde_json::json!(DefineGuildMemberRole::GuildRoleAdmin as i32)),
        point: Some(0),
        supporter_info_index: None,
        last_login_date: Some(now),
        update_date: Some(now),
        date: Some(now),
    };
    if let Err(e) = guild_member_info::add_guild_member_info(pool, &member_row).await {
        tracing::warn!("GuildCreate: failed to insert member row: {}", e);
    }

    let response = GuildCreateResponse {
        guild_info: Some(to_guild_db_info(&info_row, &base)),
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

    let (route, code) = PacketCodeType::GuildCreate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
