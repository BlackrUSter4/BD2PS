use bd2::prost::Message;
use bd2::proto::proto_net::{FishingBiteFishHpUpdateRequest, FishingBiteFishHpUpdateResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No server-authoritative combat formula exists (no relevant table captured) — stores and
/// echoes back whatever the client reports, clamped to a sane non-negative range.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingBiteFishHpUpdateRequest) -> GameResponse {
    info!("Handling FishingBiteFishHpUpdateRequest: {:?}", req);

    let hp = req.hp.unwrap_or(0).max(0);
    let _ = database::db::fishing::fishing_bite_session::set_hp(pool, uid, hp).await;

    let response = FishingBiteFishHpUpdateResponse { hp: Some(hp) };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingBiteFishHpUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
