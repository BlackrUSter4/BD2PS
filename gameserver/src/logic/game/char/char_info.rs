use bd2::prost::Message;
use bd2::proto::proto_net::{CharDbInfo, CharInfoRequest, CharInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_info::get_char_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CharInfoRequest) -> GameResponse {
    info!("Handling CharInfoRequest: {:?}", req);

    let char_rows = match get_char_info(pool, uid).await {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("get_char_info error: {:?}", err);
            vec![]
        }
    };

    let char_info = char_rows
        .into_iter()
        .map(|row| CharDbInfo {
            inven_index: row.inven_index,
            id: row.id,
            hp: row.hp,
            level: row.level,
            costume_id: row.costume_id,
            exp: row.exp,
            use_costume: row.use_costume,
            talent_level: row.talent_level,
            talent_exp: row.talent_exp,
            solidarity_reward: row.solidarity_reward,
            expiry_time: row.expiry_time,
            connect_potential_costume: row.connect_potential_costume,
            pictorialbook_info: vec![], // empty
        })
        .collect::<Vec<_>>();

    let response = CharInfoResponse {
        char_info,
        field_char_control_deck_type: Some(0),
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::CharInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
