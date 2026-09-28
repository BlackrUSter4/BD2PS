use bd2::prost::Message;
use bd2::proto::proto_net::{SpineInteractionRecordDbInfo, SpineInteractionRecordInfoRequest, SpineInteractionRecordInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::spine_interaction::spine_interaction_record;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, record_data_url};

pub async fn handle(pool: &SqlitePool, uid: i64, req: SpineInteractionRecordInfoRequest) -> GameResponse {
    info!("Handling SpineInteractionRecordInfoRequest: {:?}", req);

    let rows = spine_interaction_record::list_for_uid(pool, uid)
        .await
        .unwrap_or_default();

    let record_info = rows
        .into_iter()
        .map(|r| SpineInteractionRecordDbInfo {
            inven_index: Some(r.inven_index),
            id: Some(r.id),
            name: Some(r.name),
            record_data_s3_url: Some(record_data_url(r.inven_index)),
        })
        .collect();

    let response = SpineInteractionRecordInfoResponse { record_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::SpineInteractionRecordInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
