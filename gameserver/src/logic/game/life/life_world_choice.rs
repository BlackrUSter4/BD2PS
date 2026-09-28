use bd2::prost::Message;
use bd2::proto::proto_net::{LifeWorldChoiceRequest, LifeWorldChoiceResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// First-time world selection: sets the chosen world id and grants the starting chunk (id 1
/// is a reasonable "home plot" convention — `LifeWorldTable` would confirm the real starting
/// chunk set, not yet captured). No starter item/currency reward is fabricated.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeWorldChoiceRequest) -> GameResponse {
    info!("Handling LifeWorldChoiceRequest: {:?}", req);

    let mut chunk_id = Vec::new();
    if let Some(world_id) = req.world_id {
        let _ = database::db::life::life_user_info::set_world_id(pool, uid, world_id).await;
        if database::db::life::life_chunk_info::add_if_missing(pool, uid, 1)
            .await
            .unwrap_or(false)
        {
            chunk_id.push(1);
        }
    }

    let response = LifeWorldChoiceResponse {
        reward_info_bundle: None,
        chunk_id,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeWorldChoice.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
