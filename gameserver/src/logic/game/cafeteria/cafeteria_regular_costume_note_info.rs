use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaRegularCostumeNoteDbInfo, CafeteriaRegularCostumeNoteInfoRequest,
    CafeteriaRegularCostumeNoteInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::cafeteria::cafeteria_regular_costume_note_info as note_db;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaRegularCostumeNoteInfoRequest,
) -> GameResponse {
    info!("Handling CafeteriaRegularCostumeNoteInfoRequest: {:?}", req);

    let notes = note_db::get_cafeteria_regular_costume_note_info(pool, uid)
        .await
        .unwrap_or_default();

    let response = CafeteriaRegularCostumeNoteInfoResponse {
        regular_costume_info: notes
            .into_iter()
            .map(|n| CafeteriaRegularCostumeNoteDbInfo {
                costume_id: n.costume_id,
                serve_count: n.serve_count,
                is_received_reward: n.is_received_reward,
            })
            .collect(),
        // No table anywhere marks a costume as "delayed visibility" — always empty.
        delayed_visibility_costume_id: vec![],
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaRegularCostumeNoteInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
