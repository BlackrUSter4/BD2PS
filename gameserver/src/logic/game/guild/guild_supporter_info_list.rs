use bd2::prost::Message;
use bd2::proto::proto_net::{GuildSupporterInfo as GuildSupporterInfoProto, GuildSupporterInfoListRequest, GuildSupporterInfoListResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_supporter_info;
use sqlx::SqlitePool;
use tracing::info;

/// Simplification: lists the caller's own supporter slots rather than every
/// guild member's (would need enumerating the whole roster's Uids) — real
/// for the realistic solo/small-guild case, worth widening later.
pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildSupporterInfoListRequest) -> GameResponse {
    info!("Handling GuildSupporterInfoListRequest: {:?}", req);

    let rows = guild_supporter_info::get_guild_supporter_info(pool, uid)
        .await
        .unwrap_or_default();
    let mut supporter_info = Vec::with_capacity(rows.len());
    for s in rows {
        let owner = s.owner_index.unwrap_or(uid);
        supporter_info.push(GuildSupporterInfoProto {
            owner_index: s.owner_index,
            slot_index: s.slot_index,
            user_id: Some(crate::logic::game::display_name(pool, owner).await),
            battle_use_count: s.battle_use_count,
            supporter_char_info_proto: s.supporter_char_info_proto,
        });
    }

    let response = GuildSupporterInfoListResponse { supporter_info };
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
    let (route, code) = PacketCodeType::GuildSupporterList.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
