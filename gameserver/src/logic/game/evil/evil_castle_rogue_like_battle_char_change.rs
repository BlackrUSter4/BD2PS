use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeBattleCharChangeRequest, EvilCastleRogueLikeBattleCharChangeResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::evil_castle_rogue_like_deck_info;
use sqlx::SqlitePool;
use tracing::info;

/// Swaps one deck slot's character mid-run — real deck mutation.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeBattleCharChangeRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeBattleCharChangeRequest: {:?}", req);

    if let (Some(out_idx), Some(in_idx)) = (req.deck_out_char_index, req.deck_in_char_index) {
        let mut deck = evil_castle_rogue_like_deck_info::get(pool, uid).await.unwrap_or_default();
        for d in deck.iter_mut() {
            if d.char_inven_index == out_idx {
                d.char_inven_index = in_idx;
            }
        }
        let entries: Vec<(i64, i32, i32)> = deck.into_iter().map(|d| (d.char_inven_index, d.position, d.sequence)).collect();
        let _ = evil_castle_rogue_like_deck_info::save(pool, uid, &entries).await;
    }

    let response = EvilCastleRogueLikeBattleCharChangeResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeBattleCharChange.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
