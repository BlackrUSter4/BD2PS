use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeGiveUpInfoRequest, EvilCastleRogueLikeGiveUpInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::evil_castle_rogue_like_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeGiveUpInfoRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeGiveUpInfoRequest: {:?}", req);

    let obsidian = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten().and_then(|r| r.obsidian).unwrap_or(0);

    let response = EvilCastleRogueLikeGiveUpInfoResponse { obsidian: Some(obsidian) };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeGiveUpInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
