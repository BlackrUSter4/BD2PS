use bd2::prost::Message;
use bd2::proto::proto_net::{GuildSupporterAddRequest, GuildSupporterAddResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_supporter_info;
use database::models::game::guild::guild_supporter_info::GuildSupporterInfo;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildSupporterAddRequest) -> GameResponse {
    info!("Handling GuildSupporterAddRequest: {:?}", req);

    let row = GuildSupporterInfo {
        index: 0,
        uid,
        owner_index: Some(uid),
        slot_index: req.slot_index,
        user_id: Some(uid.to_string()),
        battle_use_count: Some(0),
        // Not a real serialized CostumeBaseDbInfo — just enough to remember which
        // char/costume this slot points at until a real proto encoding is needed.
        supporter_char_info_proto: Some(
            serde_json::json!({"charInvenIndex": req.char_inven_index, "costumeInvenIndex": req.costume_inven_index}).to_string(),
        ),
    };
    if let Err(e) = guild_supporter_info::add_guild_supporter_info(pool, &row).await {
        tracing::warn!("GuildSupporterAdd: failed to insert: {}", e);
    }

    let response = GuildSupporterAddResponse {};
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
    let (route, code) = PacketCodeType::GuildSupporterAdd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
