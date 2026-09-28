use bd2::prost::Message;
use bd2::proto::proto_net::{SpineInteractionRecordDetailInfoRequest, SpineInteractionRecordDetailInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::spine_interaction::spine_interaction_record;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: SpineInteractionRecordDetailInfoRequest,
) -> GameResponse {
    info!("Handling SpineInteractionRecordDetailInfoRequest: {:?}", req);

    let inven_index = req.inven_index.unwrap_or_default();
    let record_data = spine_interaction_record::get(pool, uid, inven_index)
        .await
        .ok()
        .flatten()
        .and_then(|r| r.record_data);

    let response = SpineInteractionRecordDetailInfoResponse { record_data };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::SpineInteractionRecordDetailInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
