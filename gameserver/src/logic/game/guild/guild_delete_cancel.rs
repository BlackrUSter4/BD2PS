use bd2::prost::Message;
use bd2::proto::proto_net::{GuildDeleteCancelRequest, GuildDeleteCancelResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::common::my_guild;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildDeleteCancelRequest) -> GameResponse {
    info!("Handling GuildDeleteCancelRequest: {:?}", req);

    if let Some((_info, base)) = my_guild(pool, uid).await {
        let guild_id = base.id.unwrap_or_default();
        let _ = sqlx::query("UPDATE GuildInfo SET DeleteRemainingTime = NULL WHERE GuildBaseInfoIndex = ?")
            .bind(guild_id)
            .execute(pool)
            .await;
    }

    let response = GuildDeleteCancelResponse {};
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

    let (route, code) = PacketCodeType::GuildDeleteCancel.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
