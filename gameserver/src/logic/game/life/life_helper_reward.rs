use super::{helper_row_to_dbinfo, now_ms};
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeHelperRewardRequest, LifeHelperRewardResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Collecting resets every assigned helper's work timer (so the state machine keeps moving),
/// but the actual yield is unknown without `LifeGatheringObjectTable` — reward left empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeHelperRewardRequest) -> GameResponse {
    info!("Handling LifeHelperRewardRequest: {:?}", req);

    let helpers = database::db::life::life_helper_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let now = now_ms();
    let mut updated = Vec::new();

    for h in helpers {
        if let (Some(work_type), Some(work_id)) = (h.work_type, h.work_id) {
            let _ =
                database::db::life::life_helper_info::assign_work(pool, h.index, work_type, work_id, now)
                    .await;
            let mut refreshed = h;
            refreshed.assign_date = Some(now);
            updated.push(helper_row_to_dbinfo(&refreshed));
        }
    }

    let response = LifeHelperRewardResponse {
        reward_bundle: None,
        helper_info: updated,
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

    let (route, code) = PacketCodeType::LifeHelperReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
