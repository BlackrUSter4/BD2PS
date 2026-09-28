use bd2::prost::Message;
use bd2::proto::proto_net::{DeckDbInfo, GuildRaidDeckInfoRequest, GuildRaidDeckInfoResponse, GuildRaidSupporterDeckDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_raid_deck_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidDeckInfoRequest) -> GameResponse {
    info!("Handling GuildRaidDeckInfoRequest: {:?}", req);

    let row = guild_raid_deck_info::get_guild_raid_deck_info(pool, uid)
        .await
        .ok()
        .and_then(|v| v.into_iter().next());

    let deck_info: Vec<DeckDbInfo> = row
        .as_ref()
        .and_then(|r| r.deck_info_index.as_deref())
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();
    let supporter_deck_info: Vec<GuildRaidSupporterDeckDbInfo> = row
        .as_ref()
        .and_then(|r| r.supporter_deck_info_index.as_deref())
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();
    let is_supporter_deck_update = row.as_ref().and_then(|r| r.is_supporter_deck_update).map(|v| v != 0);

    let response = GuildRaidDeckInfoResponse { deck_info, supporter_deck_info, is_supporter_deck_update };
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
    let (route, code) = PacketCodeType::GuildRaidDeckInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
