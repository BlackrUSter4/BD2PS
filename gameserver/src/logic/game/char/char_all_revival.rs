use bd2::prost::Message;
use bd2::proto::proto_net::{CharAllRevivalRequest, CharAllRevivalResponse, CharDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_info;
use sqlx::SqlitePool;
use tracing::info;

/// Same "no MaxHp stored" placeholder as CharHealing.
const PLACEHOLDER_FULL_HP: i64 = 999_999;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CharAllRevivalRequest) -> GameResponse {
    info!("Handling CharAllRevivalRequest: {:?}", req);

    let mut char_info_out = Vec::new();
    for inven_index in &req.char_index {
        let Ok(Some(row)) = char_info::get_by_inven_index(pool, uid, *inven_index).await else {
            continue;
        };
        let _ = char_info::set_hp_by_inven(pool, uid, *inven_index, PLACEHOLDER_FULL_HP).await;
        char_info_out.push(CharDbInfo {
            inven_index: row.inven_index,
            id: row.id,
            hp: Some(PLACEHOLDER_FULL_HP),
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
        });
    }

    let response = CharAllRevivalResponse {
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
    let (route, code) = PacketCodeType::CharAllRevival.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
