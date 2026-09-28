use bd2::prost::Message;
use bd2::proto::proto_net::{EventExchangeDbInfo, EventExchangeInfoRequest, EventExchangeInfoResponse, EventExchangeRewardDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::event::{event_exchange_info as ex_db, event_exchange_reward_info as reward_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account exchange progress (page/key_count) and reward-claim counts.
/// Legitimately empty until the account has actually opened/exchanged in some event — no
/// event-exchange master data exists anywhere in this project to synthesize starting state.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EventExchangeInfoRequest) -> GameResponse {
    info!("Handling EventExchangeInfoRequest: {:?}", req);

    let event_exchange_info = ex_db::get_event_exchange_info(pool, uid).await.unwrap_or_default()
        .into_iter()
        .map(|r| EventExchangeDbInfo { event_uid: r.event_uid, group_id: r.group_id, page: r.page, key_count: r.key_count })
        .collect();

    let event_exchange_reward_info = reward_db::get_event_exchange_reward_info(pool, uid).await.unwrap_or_default()
        .into_iter()
        .map(|r| EventExchangeRewardDbInfo { event_uid: r.event_uid, group_id: r.group_id, id: r.id, count: r.count })
        .collect();

    let response = EventExchangeInfoResponse { event_exchange_info, event_exchange_reward_info };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EventExchangeInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
