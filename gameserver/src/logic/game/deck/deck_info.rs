use bd2::prost::Message;
use bd2::proto::proto_net::{DeckDbInfo, DeckInfoRequest, DeckInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::deck::deck_info::get_deck_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, _req: DeckInfoRequest) -> GameResponse {
    info!("Handling DeckInfoRequest for uid: {}", uid);

    // Fetch deck info from DB
    let deck_infos = match get_deck_info(pool, uid).await {
        Ok(list) => list,
        Err(err) => {
            eprintln!("Error fetching DeckInfo: {:?}", err);
            vec![]
        }
    };

    // Map DB model → protobuf structure
    let deck_info_proto: Vec<DeckDbInfo> = deck_infos
        .into_iter()
        .map(|d| DeckDbInfo {
            char_inven_index: d.char_inven_index,
            position: d.position,
            sequence: d.sequence,
        })
        .collect();

    // Talent slot info (defaults to 4 slots = 0)
    let talent_slot_info = vec![0, 0, 0, 0];

    // Construct final response
    let response = DeckInfoResponse {
        deck_info: deck_info_proto,
        talent_slot_info,
        ..Default::default()
    };

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

    let (route, code) = PacketCodeType::DeckInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
