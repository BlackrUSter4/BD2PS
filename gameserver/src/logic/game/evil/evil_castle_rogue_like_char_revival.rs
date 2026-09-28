use bd2::prost::Message;
use bd2::proto::proto_net::{CharDbInfo, EvilCastleRogueLikeCharRevivalRequest, EvilCastleRogueLikeCharRevivalResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_info;
use sqlx::SqlitePool;
use tracing::info;

/// Revives every fallen (Hp == 0) character in the account's roster —
/// real HP restore, no cost gating found in this request's own schema.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeCharRevivalRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeCharRevivalRequest: {:?}", req);

    let mut revived = vec![];
    let chars = char_info::get_char_info(pool, uid).await.unwrap_or_default();
    for mut c in chars {
        if c.hp.unwrap_or(1) <= 0 {
            c.hp = Some(1);
            let _ = char_info::set_hp(pool, uid, c.index, 1).await;
            revived.push(CharDbInfo {
                inven_index: c.inven_index,
                id: c.id,
                hp: c.hp,
                level: c.level,
                costume_id: c.costume_id,
                exp: c.exp,
                use_costume: c.use_costume,
                talent_level: c.talent_level,
                talent_exp: c.talent_exp,
                solidarity_reward: c.solidarity_reward,
                expiry_time: c.expiry_time,
                pictorialbook_info: vec![],
                connect_potential_costume: c.connect_potential_costume,
            });
        }
    }

    let response = EvilCastleRogueLikeCharRevivalResponse { char_info: revived };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeCharRevival.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
