use bd2::prost::Message;
use bd2::proto::proto_net::{FishingRodSetRequest, FishingRodSetResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingRodSetRequest) -> GameResponse {
    info!("Handling FishingRodSetRequest: {:?}", req);

    let mut equipped = None;
    if let Some(idx) = req.rod_inven_index {
        let owned = database::db::fishing::fishing_rod_info::get_by_index(pool, uid, idx)
            .await
            .ok()
            .flatten()
            .is_some();
        if owned {
            let _ = database::db::fishing::fishing_user_info::set_use_rod(pool, uid, idx).await;
            equipped = Some(idx);
        }
    }

    let response = FishingRodSetResponse { equipped_rod_inven_index: equipped };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingRodSet.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
