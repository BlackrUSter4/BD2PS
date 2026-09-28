use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumPresetInfoChangeRequest, ColosseumPresetInfoChangeResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_preset_info as preset_db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumPresetInfoChangeRequest) -> GameResponse {
    info!("Handling ColosseumPresetInfoChangeRequest: {:?}", req);

    let slot = req.slot.unwrap_or(0);
    if preset_db::get_by_uid_and_slot(pool, uid, slot).await.ok().flatten().is_some() {
        let _ = preset_db::update_info(
            pool,
            uid,
            slot,
            req.preset_name.as_deref(),
            req.preset_resource_id,
            req.preset_resource_color,
        )
        .await;
    } else {
        let _ = preset_db::insert(
            pool,
            uid,
            slot,
            req.preset_name.as_deref(),
            req.preset_resource_id,
            req.preset_resource_color,
        )
        .await;
    }

    let response = ColosseumPresetInfoChangeResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumPresetInfoChange.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
