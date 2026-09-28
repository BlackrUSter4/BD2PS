use bd2::prost::Message;
use bd2::proto::proto_net::{SpineInteractionRecordSaveRequest, SpineInteractionRecordSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::spine_interaction::spine_interaction_record;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, record_data_url};

pub async fn handle(pool: &SqlitePool, uid: i64, req: SpineInteractionRecordSaveRequest) -> GameResponse {
    info!("Handling SpineInteractionRecordSaveRequest: {:?}", req);

    let id = req.id.unwrap_or_default();
    let data = req.record_data.unwrap_or_default();
    let inven_index = spine_interaction_record::insert(pool, uid, id, &data)
        .await
        .unwrap_or_default();

    let response = SpineInteractionRecordSaveResponse {
        inven_index: Some(inven_index as i32),
        name: Some(String::new()),
        record_data_s3_url: Some(record_data_url(inven_index)),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::SpineInteractionRecordSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
