use super::{now_ms, placeholder_catch};
use bd2::prost::Message;
use bd2::proto::proto_net::{FishingBiteStartCheatRequest, FishingBiteStartCheatResponse, FishingGameFishInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::models::game::fishing::fishing_bite_session::FishingBiteSession;
use sqlx::SqlitePool;
use tracing::info;

/// "Cheat" variant of BiteStart — honors the client-requested fish_id directly (still a
/// placeholder size, same caveat as FishingBiteStart).
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingBiteStartCheatRequest) -> GameResponse {
    info!("Handling FishingBiteStartCheatRequest: {:?}", req);

    let (fish_id, size) = placeholder_catch(req.fish_id);
    let _ = database::db::fishing::fishing_bite_session::upsert(
        pool,
        &FishingBiteSession {
            uid,
            fish_id,
            size,
            hp: 100,
            stamina: 100,
            start_time: Some(now_ms()),
        },
    )
    .await;

    let response = FishingBiteStartCheatResponse {
        fish_info: Some(FishingGameFishInfo { id: Some(fish_id), size: Some(size) }),
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
    let (route, code) = PacketCodeType::FishingBiteStartCheat.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
