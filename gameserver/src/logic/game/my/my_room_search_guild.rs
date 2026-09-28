use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomSearchGuildRequest, MyRoomSearchGuildResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::build_user_info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomSearchGuildRequest) -> GameResponse {
    info!("Handling MyRoomSearchGuildRequest: {:?}", req);

    let members = database::db::guild::guild_member_info::get_guild_member_info(pool, uid)
        .await
        .unwrap_or_default();

    let mut room_info = Vec::new();
    for member in members {
        if let Some(owner) = member.owner_index {
            if owner != uid {
                room_info.push(build_user_info(pool, owner).await);
            }
        }
    }

    let response = MyRoomSearchGuildResponse { room_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomSearchGuild.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
