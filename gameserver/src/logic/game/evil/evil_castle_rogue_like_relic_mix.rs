use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeRelicMixRequest, EvilCastleRogueLikeRelicMixResponse, Notify, RelicDbInfo};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::relic::relic_info;
use database::models::game::relic::relic_info::RelicInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Combines two owned relics into a new one via RLRelicMixTable's real
/// material/result mapping.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeRelicMixRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeRelicMixRequest: {:?}", req);

    let mut removed = vec![];
    let mut added = vec![];

    if req.inven_index.len() >= 2 {
        let owned = relic_info::get_relic_info(pool, uid).await.unwrap_or_default();
        let a = owned.iter().find(|r| Some(r.inven_index.unwrap_or(-1)) == Some(req.inven_index[0]));
        let b = owned.iter().find(|r| Some(r.inven_index.unwrap_or(-1)) == Some(req.inven_index[1]));
        if let (Some(a), Some(b)) = (a, b) {
            let (id_a, id_b) = (a.id.unwrap_or(0), b.id.unwrap_or(0));
            let recipe = exceldb::get().rlrelicmixtable.all().iter().find(|m| {
                (m.mix_material_relic_id1 == id_a && m.mix_material_relic_id2 == id_b)
                    || (m.mix_material_relic_id1 == id_b && m.mix_material_relic_id2 == id_a)
            });
            if let Some(recipe) = recipe {
                let _ = relic_info::delete_by_index(pool, uid, a.index).await;
                let _ = relic_info::delete_by_index(pool, uid, b.index).await;
                removed.push(RelicDbInfo { inven_index: a.inven_index, id: a.id });
                removed.push(RelicDbInfo { inven_index: b.inven_index, id: b.id });
                let new_index = chrono::Utc::now().timestamp_millis();
                let new_relic = RelicInfo { index: 0, uid, inven_index: Some(new_index), id: Some(recipe.mix_result_relic_id) };
                let _ = relic_info::insert(pool, &new_relic).await;
                added.push(RelicDbInfo { inven_index: Some(new_index), id: Some(recipe.mix_result_relic_id) });
            }
        }
    }

    let response = EvilCastleRogueLikeRelicMixResponse { add_relic_info: added, remove_relic_info: removed };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeRelicMix.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
