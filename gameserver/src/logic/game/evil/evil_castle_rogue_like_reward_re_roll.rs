use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeChoiceInfo as ChoiceInfoMsg, EvilCastleRogueLikeRewardReRollRequest, EvilCastleRogueLikeRewardReRollResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::evil::{evil_castle_rogue_like_choice_info, evil_castle_rogue_like_info};
use sqlx::SqlitePool;
use tracing::info;

/// Re-rolls the current reward choice (a fresh set of relic ids) — costs
/// one of the account's remaining re_roll count.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeRewardReRollRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeRewardReRollRequest: {:?}", req);
    let _ = req;

    let mut count = 0;
    let mut choice_info = None;
    if let Some(mut run) = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten() {
        let remaining = run.re_roll.unwrap_or(0);
        if remaining > 0 {
            run.re_roll = Some(remaining - 1);
            let _ = evil_castle_rogue_like_info::upsert(pool, &run).await;
            count = remaining - 1;

            let relics = &exceldb::get().rlrelictable;
            let mut seed = super::roguelike::new_seed(uid);
            let ids: Vec<i32> = (0..3)
                .filter_map(|_| relics.all().get((super::roguelike::rand_u32(&mut seed) as usize) % relics.all().len().max(1)).map(|r| r.id))
                .collect();
            let _ = evil_castle_rogue_like_choice_info::upsert(pool, uid, 1, &ids).await;
            choice_info = Some(ChoiceInfoMsg { r#type: Some(1), id: ids });
        }
    }

    let response = EvilCastleRogueLikeRewardReRollResponse { count: Some(count), choice_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeRewardReRoll.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
