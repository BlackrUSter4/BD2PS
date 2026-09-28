use bd2::prost::Message;
use bd2::proto::proto_net::{MasterTitleInfoUpdateRequest, MasterTitleInfoUpdateResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::master_title::master_title_info as db;
use database::models::game::master_title::master_title_info::MasterTitleInfo;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: MasterTitleInfoUpdateRequest) -> GameResponse {
    info!("Handling MasterTitleInfoUpdateRequest: {:?}", req);

    let row = MasterTitleInfo {
        uid,
        name: req.name,
        month: req.month,
        day: req.day,
    };
    let _ = db::save(pool, &row).await;

    let response = MasterTitleInfoUpdateResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MasterTitleInfoUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
