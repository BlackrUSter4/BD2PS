use bd2::prost::Message;
use bd2::proto::proto_net::{SpineInteractionRecordNameUpdateRequest, SpineInteractionRecordNameUpdateResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::spine_interaction::spine_interaction_record;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: SpineInteractionRecordNameUpdateRequest,
) -> GameResponse {
    info!("Handling SpineInteractionRecordNameUpdateRequest: {:?}", req);

    let inven_index = req.inven_index.unwrap_or_default();
    let name = req.name.unwrap_or_default();
    let _ = spine_interaction_record::update_name(pool, uid, inven_index, &name).await;

    let response = SpineInteractionRecordNameUpdateResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::SpineInteractionRecordNameUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
