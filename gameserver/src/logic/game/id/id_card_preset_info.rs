use bd2::prost::Message;
use bd2::proto::proto_net::{IdCardPresetInfo as ProtoPreset, IdCardPresetInfoRequest, IdCardPresetInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::id::id_card_preset_info as preset_db;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: IdCardPresetInfoRequest) -> GameResponse {
    info!("Handling IdCardPresetInfoRequest: {:?}", req);

    let rows = preset_db::get_id_card_preset_info(pool, uid).await.unwrap_or_default();
    let mut preset_info = Vec::new();
    for row in rows {
        let id_card_info = match row.id_card_info_index {
            Some(idx) => match database::db::id::id_card_info::get_by_index(pool, uid, idx).await {
                Ok(card_row) => Some(super::card_to_proto(pool, uid, &card_row).await),
                Err(_) => None,
            },
            None => None,
        };
        preset_info.push(ProtoPreset { id: row.id, id_card_info });
    }

    let response = IdCardPresetInfoResponse { preset_info };

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

    let (route, code) = PacketCodeType::IdCardPresetInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
