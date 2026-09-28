use bd2::prost::Message;
use bd2::proto::proto_net::{MasterTitleInfoRequest, MasterTitleInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::master_title::master_title_info as db;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: MasterTitleInfoRequest) -> GameResponse {
    info!("Handling MasterTitleInfoRequest: {:?}", req);

    let row = db::get_or_default(pool, uid).await;

    let response = MasterTitleInfoResponse {
        name: row.name,
        month: row.month,
        day: row.day,
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MasterTitleInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
