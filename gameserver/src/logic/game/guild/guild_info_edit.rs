use bd2::prost::Message;
use bd2::proto::proto_net::{GuildInfoEditRequest, GuildInfoEditResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_base_info;
use sqlx::SqlitePool;
use tracing::info;

use super::common::{my_guild, now_ms};

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildInfoEditRequest) -> GameResponse {
    info!("Handling GuildInfoEditRequest: {:?}", req);

    if let Some((info_row, base)) = my_guild(pool, uid).await {
        let guild_id = base.id.unwrap_or_default();
        let _ = guild_base_info::update_by_guild_id(
            pool,
            guild_id,
            None,
            req.icon,
            req.icon_color.as_deref(),
        )
        .await;
        if req.message.is_some() {
            let _ = sqlx::query("UPDATE GuildInfo SET Message = ?, UpdateDate = ? WHERE GuildBaseInfoIndex = ?")
                .bind(&req.message)
                .bind(now_ms())
                .bind(guild_id)
                .execute(pool)
                .await;
        }
        if req.join_type.is_some() {
            let _ = sqlx::query("UPDATE GuildInfo SET JoinType = ? WHERE GuildBaseInfoIndex = ?")
                .bind(req.join_type)
                .bind(guild_id)
                .execute(pool)
                .await;
        }
        let _ = info_row; // silence unused warning if fields end up untouched
    }

    let response = GuildInfoEditResponse {};
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

    let (route, code) = PacketCodeType::GuildInfoEdit.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
