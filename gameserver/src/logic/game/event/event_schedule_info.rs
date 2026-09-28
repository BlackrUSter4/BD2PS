use bd2::prost::Message;
use bd2::proto::proto_net::{
    DefineDelayedVisibilityType::*, DefineEventType::*, DelayedVisibilityScheduleDbInfo,
    EventScheduleDbInfo, EventScheduleInfoRequest, EventScheduleInfoResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use serde_json::Value;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(_pool: &SqlitePool, _uid: i64, req: EventScheduleInfoRequest) -> GameResponse {
    info!("Handling EventScheduleInfoRequest: {:?}", req);

    let data: Value = serde_json::from_str(include_str!(
        "../../../../../data/starter/event_schedule_info.json"
    ))
    .unwrap();

    let event_schedule_info = data
        .get("eventScheduleInfo")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|entry| {
                    let event_type_str = entry
                        .get("eventType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("EVENT_ALWAYS_ATTENDANCE");

                    let event_type = match event_type_str {
                        "EVENT_ALWAYS_ATTENDANCE" => EventAlwaysAttendance,
                        "EVENT_LIMIT_ATTENDANCE" => EventLimitAttendance,
                        "EVENT_WORLD_BUFF" => EventWorldBuff,
                        "EVENT_MISSION" => EventMission,
                        "EVENT_PASS" => EventPass,
                        "EVENT_DROP_ITEM" => EventDropItem,
                        "EVENT_EXCHANGE" => EventExchange,
                        "EVENT_PACK" => EventPack,
                        "EVENT_BATTLE" => EventBattle,
                        "EVENT_STORY" => EventStory,
                        "EVENT_MINI_GAME" => EventMiniGame,
                        "EVENT_MINI_GAME_BOARD" => EventMiniGameBoard,
                        "EVENT_MINI_GAME_BINGO" => EventMiniGameBingo,
                        "EVENT_LOST_COIN" => EventLostCoin,
                        "EVENT_SHOP" => EventShop,
                        "EVENT_GACHA_STEP_UP" => EventGachaStepUp,
                        "EVENT_PUZZLE" => EventPuzzle,
                        "EVENT_CHARGE_COST" => EventChargeCost,
                        "EVENT_MINI_GAME_ROULETTE" => EventMiniGameRoulette,
                        _ => EventAlwaysAttendance,
                    };

                    EventScheduleDbInfo {
                        id: entry.get("id").and_then(|v| v.as_i64()).map(|v| v as i32),
                        event_type: Some(event_type.into()),
                        event_id: entry
                            .get("eventId")
                            .and_then(|v| v.as_i64())
                            .map(|v| v as i32),
                        event_sub_id: entry
                            .get("eventSubId")
                            .and_then(|v| v.as_i64())
                            .map(|v| v as i32),
                        start_date: entry.get("startDate").and_then(|v| v.as_i64()),
                        end_date: entry.get("endDate").and_then(|v| v.as_i64()),
                        is_active: entry.get("isActive").and_then(|v| v.as_bool()),
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let delayed_visibility_schedule_info = data
        .get("delayedVisibilityScheduleInfo")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|entry| {
                    let type_str = entry
                        .get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("DBT_None");

                    let vtype = match type_str {
                        "CharacterPictorialBookTable" => CharacterPictorialBookTable,
                        "CostumePictorialBookTable" => CostumePictorialBookTable,
                        "EquipmentPictorialBookTable" => EquipmentPictorialBookTable,
                        "TalentPictorialBookTable" => TalentPictorialBookTable,
                        "CostumeDesignTable" => CostumeDesignTable,
                        "MercenaryScoutTable" => MercenaryScoutTable,
                        "CafeteriaCostumeTable" => CafeteriaCostumeTable,
                        "CostumeDesignTable_SeasonEvent" => CostumeDesignTableSeasonEvent,
                        "CostumeDesignTable_RogueLike" => CostumeDesignTableRogueLike,
                        "IdCardItemTable" => IdCardItemTable,
                        "AchievementTable" => AchievementTable,
                        _ => DbtNone,
                    };

                    DelayedVisibilityScheduleDbInfo {
                        r#type: Some(vtype.into()),
                        id: entry.get("id").and_then(|v| v.as_i64()).map(|v| v as i32),
                        table_id: entry
                            .get("tableId")
                            .and_then(|v| v.as_i64())
                            .map(|v| v as i32),
                        open_date: entry.get("openDate").and_then(|v| v.as_i64()),
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let response = EventScheduleInfoResponse {
        event_schedule_info,
        delayed_visibility_schedule_info,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EventScheduleInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
