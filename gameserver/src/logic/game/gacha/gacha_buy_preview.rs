use bd2::prost::Message;
use bd2::proto::proto_net::{CharDbInfo, GachaBuyPreviewRequest, GachaBuyPreviewResponse, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use rand::seq::IndexedRandom;
use sqlx::SqlitePool;
use tracing::info;

/// A preview draw — shows what a purchase WOULD grant without actually
/// spending anything or persisting a character (unlike GachaBuy). Same
/// "real random real-CharTable id, no weighted-rate data exists" judgment
/// call as GachaBuy.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: GachaBuyPreviewRequest) -> GameResponse {
    info!("Handling GachaBuyPreviewRequest: {:?}", req);

    let picked = exceldb::get().chartable.all().choose(&mut rand::thread_rng());
    let preview_item_info = picked.map(|c| RewardDbInfoBundle {
        char_info: vec![CharDbInfo {
            id: Some(c.id),
            costume_id: Some(c.default_costume_id),
            level: Some(1),
            ..Default::default()
        }],
        ..Default::default()
    });

    let response = GachaBuyPreviewResponse { preview_item_info };
    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::GachaBuyPreview.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
