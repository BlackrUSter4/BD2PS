use bd2::prost::Message;
use bd2::proto::proto_net::{LifeHelperRewardInfoRequest, LifeHelperRewardInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// The request carries no helper reference (just `seq`), so this reports on whichever
/// currently-assigned helper has been working longest — a reasonable proxy for "next reward
/// due", though the actual yield amount depends on `LifeGatheringObjectTable`/work-type yield
/// data this server hasn't captured, so the reward bundle itself is left empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeHelperRewardInfoRequest) -> GameResponse {
    info!("Handling LifeHelperRewardInfoRequest: {:?}", req);

    let helpers = database::db::life::life_helper_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let assign_time = helpers.iter().filter_map(|h| h.assign_date).min();

    let response = LifeHelperRewardInfoResponse {
        assign_time,
        reward_bundle: None,
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

    let (route, code) = PacketCodeType::LifeHelperRewardInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
