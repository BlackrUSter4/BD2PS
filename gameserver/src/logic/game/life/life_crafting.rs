use super::try_consume_items;
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeCraftingRequest, LifeCraftingResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Same situation as Cooking: consumes materials for real, but the output item comes from
/// `LifeCraftingObjectTable`, which has no captured master data yet — reward left empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeCraftingRequest) -> GameResponse {
    info!("Handling LifeCraftingRequest: {:?}", req);

    let _consumed = try_consume_items(pool, uid, &req.use_item_info).await;

    let response = LifeCraftingResponse {
        reward_info_bundle: None,
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

    let (route, code) = PacketCodeType::LifeCrafting.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
