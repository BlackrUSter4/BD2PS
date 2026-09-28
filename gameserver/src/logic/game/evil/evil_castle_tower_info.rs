use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleTowerDbInfo, EvilCastleTowerInfoRequest, EvilCastleTowerInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use sqlx::SqlitePool;
use tracing::info;

/// Static tower list, read straight from EvilCastleTable (each row is a
/// boss/floor checkpoint) — no per-account state needed here.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: EvilCastleTowerInfoRequest) -> GameResponse {
    info!("Handling EvilCastleTowerInfoRequest: {:?}", req);

    let info = exceldb::get()
        .evilcastletable
        .all()
        .iter()
        .map(|t| EvilCastleTowerDbInfo {
            id: Some(t.id),
            battle_challenge_index: t.monster_id.clone(),
            battle_mode: None,
        })
        .collect();

    let response = EvilCastleTowerInfoResponse { info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleTowerInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
