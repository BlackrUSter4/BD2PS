use super::{item_deltas, now_ms};
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeEatFoodDbInfo, LifeEatFoodRequest, LifeEatFoodResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::models::game::life::life_duration_buff_info::LifeDurationBuffInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Placeholder duration: the real per-food buff length lives in a food/buff master table this
/// server hasn't captured for the Life-specific food items (the pre-existing FoodTable is a
/// different, already-implemented system). 30 minutes is a reasonable generic placeholder.
const PLACEHOLDER_BUFF_DURATION_MS: i64 = 30 * 60 * 1000;

pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeEatFoodRequest) -> GameResponse {
    info!("Handling LifeEatFoodRequest: {:?}", req);

    let deltas = item_deltas(&req.use_item);
    let mut eat_food_info = Vec::new();
    let now = now_ms();

    for d in deltas {
        if database::db::item::item_info::consume(pool, uid, d.id, d.count)
            .await
            .unwrap_or(false)
        {
            let end_time = now + PLACEHOLDER_BUFF_DURATION_MS;
            let _ = database::db::life::life_duration_buff_info::insert(
                pool,
                &LifeDurationBuffInfo {
                    index: 0,
                    uid,
                    item_id: Some(d.id),
                    end_time: Some(end_time),
                },
            )
            .await;
            eat_food_info.push(LifeEatFoodDbInfo {
                item_id: Some(d.id),
                end_time: Some(end_time),
            });
        }
    }

    let response = LifeEatFoodResponse { eat_food_info };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeEatFood.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
