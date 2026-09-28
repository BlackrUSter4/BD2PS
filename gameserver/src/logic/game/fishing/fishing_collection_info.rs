use super::collection_row_to_dbinfo;
use bd2::prost::Message;
use bd2::proto::proto_net::{FishingCollectionInfoRequest, FishingCollectionInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingCollectionInfoRequest) -> GameResponse {
    info!("Handling FishingCollectionInfoRequest: {:?}", req);

    let rows = database::db::fishing::fishing_collection_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();

    let response = FishingCollectionInfoResponse {
        collection_info: rows.iter().map(collection_row_to_dbinfo).collect(),
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
    let (route, code) = PacketCodeType::FishingCollectionInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
