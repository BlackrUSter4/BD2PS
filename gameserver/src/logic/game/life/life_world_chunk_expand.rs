use super::{life_user_to_dbinfo, try_consume_items};
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeWorldChunkExpandRequest, LifeWorldChunkExpandResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Consumes the claimed cost for real and grants the requested chunk — the actual expansion
/// cost table (`LifeChunkExpandTable`) isn't captured, so this trusts the client's own
/// use_item_info as payment rather than validating it server-side.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeWorldChunkExpandRequest) -> GameResponse {
    info!("Handling LifeWorldChunkExpandRequest: {:?}", req);

    if let Some(chunk_id) = req.chunk_id {
        if try_consume_items(pool, uid, &req.use_item_info).await {
            let _ = database::db::life::life_chunk_info::add_if_missing(pool, uid, chunk_id).await;
        }
    }

    let user = database::db::life::life_user_info::get_or_create(pool, uid)
        .await
        .unwrap_or_default();
    let chunks = database::db::life::life_chunk_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let life_user_info = Some(life_user_to_dbinfo(
        &user,
        chunks.iter().map(|c| c.chunk_id).collect(),
    ));

    let response = LifeWorldChunkExpandResponse { life_user_info };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeWorldChunkExpand.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
