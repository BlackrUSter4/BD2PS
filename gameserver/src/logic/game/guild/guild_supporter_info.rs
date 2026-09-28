use bd2::prost::Message;
use bd2::proto::proto_net::{GuildSupporterInfo as GuildSupporterInfoProto, GuildSupporterInfoRequest, GuildSupporterInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, _uid: i64, req: GuildSupporterInfoRequest) -> GameResponse {
    info!("Handling GuildSupporterInfoRequest: {:?}", req);

    let supporter_info = if let (Some(owner), Some(slot)) = (req.member_owner_index, req.slot_index) {
        let row = sqlx::query_as::<_, database::models::game::guild::guild_supporter_info::GuildSupporterInfo>(
            "SELECT * FROM GuildSupporterInfo WHERE OwnerIndex = ? AND SlotIndex = ?",
        )
        .bind(owner)
        .bind(slot)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten();
        match row {
            Some(s) => Some(GuildSupporterInfoProto {
                owner_index: s.owner_index,
                slot_index: s.slot_index,
                user_id: Some(crate::logic::game::display_name(pool, owner).await),
                battle_use_count: s.battle_use_count,
                supporter_char_info_proto: s.supporter_char_info_proto,
            }),
            None => None,
        }
    } else {
        None
    };

    let response = GuildSupporterInfoResponse { supporter_info };
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
    let (route, code) = PacketCodeType::GuildSupporterInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
