use bd2::prost::Message;
use bd2::proto::proto_net::{SpineInteractionRecordDataUpdateRequest, SpineInteractionRecordDataUpdateResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::spine_interaction::spine_interaction_record;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, record_data_url};

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: SpineInteractionRecordDataUpdateRequest,
) -> GameResponse {
    info!("Handling SpineInteractionRecordDataUpdateRequest: {:?}", req);

    let inven_index = req.inven_index.unwrap_or_default();
    let data = req.record_data.unwrap_or_default();
    let _ = spine_interaction_record::update_data(pool, uid, inven_index, &data).await;

    let response = SpineInteractionRecordDataUpdateResponse {
        record_data_s3_url: Some(record_data_url(inven_index)),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::SpineInteractionRecordDataUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
