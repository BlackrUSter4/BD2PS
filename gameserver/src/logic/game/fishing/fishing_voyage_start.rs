use super::now_ms;
use bd2::prost::Message;
use bd2::proto::proto_net::{FishingVoyageStartRequest, FishingVoyageStartResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No real multiplayer backend exists, so voyages are always solo (`is_multi_host: false`).
/// The cooldown reuses `multiSearchTime` from `FishingDefaultTable` (real captured data,
/// though that field actually describes room-search timing, not a voyage-AP cooldown — the
/// closest genuine value available, flagged as a repurposed placeholder).
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingVoyageStartRequest) -> GameResponse {
    info!("Handling FishingVoyageStartRequest: {:?}", req);

    let now = now_ms();
    let user = database::db::fishing::fishing_user_info::get_or_create(pool, uid)
        .await
        .unwrap_or_default();
    let ready = user.multi_ap_reset_time.map(|t| now >= t).unwrap_or(true);

    if ready {
        let default_row = data::exceldb::get().fishingdefaulttable.all().first().cloned();
        let cooldown_ms = default_row.map(|d| d.multi_search_time as i64 * 1000).unwrap_or(600_000);
        let _ = database::db::fishing::fishing_user_info::set_multi_ap_reset_time(pool, uid, now + cooldown_ms).await;
    }

    let response = FishingVoyageStartResponse { is_multi_host: Some(false) };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingVoyageStart.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
