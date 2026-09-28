use bd2::prost::Message;
use bd2::proto::proto_net::{DeckDbInfo, EvilCastleRogueLikeDeckSaveRequest, EvilCastleRogueLikeDeckSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::evil_castle_rogue_like_deck_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeDeckSaveRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeDeckSaveRequest: {:?}", req);

    let entries: Vec<(i64, i32, i32)> = req
        .deck_info
        .iter()
        .filter_map(|d| Some((d.char_inven_index?, d.position.unwrap_or(0), d.sequence.unwrap_or(0))))
        .collect();
    let _ = evil_castle_rogue_like_deck_info::save(pool, uid, &entries).await;

    let deck_info: Vec<DeckDbInfo> = entries
        .into_iter()
        .map(|(c, p, s)| DeckDbInfo { char_inven_index: Some(c), position: Some(p), sequence: Some(s) })
        .collect();

    let response = EvilCastleRogueLikeDeckSaveResponse { deck_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeDeckSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
