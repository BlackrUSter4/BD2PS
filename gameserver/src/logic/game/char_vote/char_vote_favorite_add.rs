use bd2::prost::Message;
use bd2::proto::proto_net::{CharVoteFavoriteAddRequest, CharVoteFavoriteAddResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CharVoteFavoriteAddRequest) -> GameResponse {
    info!("Handling CharVoteFavoriteAddRequest: {:?}", req);

    if let Some(candidate_id) = req.candidate_id {
        let _ = database::db::char_vote::char_vote_favorite::add(pool, uid, candidate_id).await;
    }

    let response = CharVoteFavoriteAddResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CharVoteFavoriteAdd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
