use bd2::prost::Message;
use bd2::proto::proto_net::{CharAwakeDbInfo, CharAwakeInfoRequest, CharAwakeInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_awake_info::get_char_awake_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CharAwakeInfoRequest) -> GameResponse {
    info!("Handling CharAwakeInfoRequest: {:?}", req);

    let rows = get_char_awake_info(pool, uid).await.unwrap_or_default();
    let char_awake_info = rows
        .into_iter()
        .map(|r| CharAwakeDbInfo {
            unique_char_id: r.unique_char_id,
            imprint_slot_1_level: r.imprint_slot_1_level,
            imprint_slot_2_level: r.imprint_slot_2_level,
            imprint_slot_3_level: r.imprint_slot_3_level,
            is_awake: r.is_awake,
            owner_index: None,
        })
        .collect::<Vec<_>>();

    let response = CharAwakeInfoResponse { char_awake_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    // NOTE: the enum variant for this route is named `CharImprintInfo`, not
    // `CharAwakeInfo` — a naming quirk baked into the original scaffold,
    // same class of thing as EvilCastle round's `EvilCastleRakingList`.
    let (route, code) = PacketCodeType::CharImprintInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
