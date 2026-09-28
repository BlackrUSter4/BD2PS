use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaDailyConnectionCostumeRefreshRequest, CafeteriaDailyConnectionCostumeRefreshResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::cafeteria::cafeteria_info;
use rand::seq::IndexedRandom;
use sqlx::SqlitePool;
use tracing::info;

/// Request carries no cost/target — a free re-roll of which costume gives the "phone number"
/// unlock today, picked from the real `CafeteriaCostumeTable` catalog.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaDailyConnectionCostumeRefreshRequest,
) -> GameResponse {
    info!("Handling CafeteriaDailyConnectionCostumeRefreshRequest: {:?}", req);

    let mut info_row = match cafeteria_info::get_or_create(pool, uid).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("CafeteriaDailyConnectionCostumeRefresh get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let costume_ids: Vec<i32> = data::exceldb::get()
        .cafeteriacostumetable
        .all()
        .iter()
        .map(|c| c.costume_id)
        .collect();
    let chosen = costume_ids.choose(&mut rand::thread_rng()).copied();

    info_row.daily_connection_costume_id = chosen;
    info_row.can_get_phone_number = Some(1);
    if let Err(e) = cafeteria_info::update_cafeteria_info(pool, &info_row).await {
        tracing::error!("CafeteriaDailyConnectionCostumeRefresh update failed: {}", e);
    }

    let response = CafeteriaDailyConnectionCostumeRefreshResponse {
        daily_connection_costume_id: info_row.daily_connection_costume_id,
        can_get_phone_number: info_row.can_get_phone_number.map(|v| v != 0),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaDailyConnectionCostumeRefresh.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
