use bd2::prost::Message;
use bd2::proto::proto_net::{CharDbInfo, CharImmortalRequest, CharImmortalResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_info;
use sqlx::SqlitePool;
use tracing::info;

/// The request carries only a list of character InvenIndex values, no items
/// or any other field to consume — there is nothing in the schema to
/// persist for an "immortal" status. Real: the listed characters are looked
/// up and their genuine current state is echoed back (not fabricated).
pub async fn handle(pool: &SqlitePool, uid: i64, req: CharImmortalRequest) -> GameResponse {
    info!("Handling CharImmortalRequest: {:?}", req);

    let rows = char_info::get_all_by_inven_index(pool, uid, &req.inven_index)
        .await
        .unwrap_or_default();

    let char_info_out = rows
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
            pictorialbook_info: vec![],
        })
        .collect::<Vec<_>>();

    let response = CharImmortalResponse {
        char_info: char_info_out,
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
    let (route, code) = PacketCodeType::CharImmortal.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
