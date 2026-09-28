// Auto-generated starter data from proto schemas - Single Transaction Version
use serde_json::Value;
use sqlx::{Sqlite, SqlitePool, Transaction};
use tracing;

/// Load CharInfo starter data from char_info.json
pub async fn load_char_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/char_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_char_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    let arr = match data.get("charInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_char_info: missing or invalid 'charInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let inven_index = entry
            .get("invenIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let id = entry.get("id").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
        let hp = entry.get("hp").and_then(|v| v.as_i64()).unwrap_or_default();
        let level = entry
            .get("level")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let costume_id = entry
            .get("costumeId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let exp = entry
            .get("exp")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let use_costume = entry
            .get("useCostume")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let talent_level = entry
            .get("talentLevel")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let talent_exp = entry
            .get("talentExp")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let solidarity_reward = entry
            .get("solidarityReward")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let expiry_time = entry
            .get("expiryTime")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        // Handle repeated nested PictorialBookInfo - extract InvenIndex values
        let pictorialbook_info_index =
            if let Some(nested_arr) = entry.get("pictorialbookInfo").and_then(|v| v.as_array()) {
                let indices: Vec<i64> = nested_arr
                    .iter()
                    .filter_map(|item| item.get("invenIndex").and_then(|v| v.as_i64()))
                    .collect();

                if indices.is_empty() {
                    None
                } else {
                    Some(serde_json::to_string(&indices).unwrap())
                }
            } else {
                None
            };
        let connect_potential_costume = entry
            .get("connectPotentialCostume")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO CharInfo (
    Uid,
    InvenIndex,
    Id,
    Hp,
    Level,
    CostumeId,
    Exp,
    UseCostume,
    TalentLevel,
    TalentExp,
    SolidarityReward,
    ExpiryTime,
    PictorialbookInfoIndex,
    ConnectPotentialCostume
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&inven_index)
        .bind(&id)
        .bind(&hp)
        .bind(&level)
        .bind(&costume_id)
        .bind(&exp)
        .bind(&use_costume)
        .bind(&talent_level)
        .bind(&talent_exp)
        .bind(&solidarity_reward)
        .bind(&expiry_time)
        .bind(&pictorialbook_info_index)
        .bind(&connect_potential_costume)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

/// Load CostumeInfo starter data from costume_info.json
pub async fn load_costume_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/costume_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_costume_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    let array = match data.get("costumeInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!(
                "load_costume_info: invalid or missing 'costumeInfo' array in costume_info.json"
            );
            return Ok(());
        }
    };

    for entry in array {
        sqlx::query(
            r#"INSERT INTO CostumeInfo (
                Uid, InvenIndex, Id, Level, UseChar, SortId, UseMyRoomCount, DesignId            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?            )"#
        )
        .bind(uid)
        .bind(entry.get("invenIndex").and_then(|v| v.as_i64()))
        .bind(entry.get("id").and_then(|v| v.as_i64()))
        .bind(entry.get("level").and_then(|v| v.as_i64()))
        .bind(entry.get("useChar").and_then(|v| v.as_i64()))
        .bind(entry.get("sortId").and_then(|v| v.as_i64()))
        .bind(entry.get("useMyRoomCount").and_then(|v| v.as_i64()))
        .bind(entry.get("designId").and_then(|v| v.as_i64()))
        .execute(&mut **tx)  // Changed from pool to tx
        .await?;
    }

    Ok(())
}

/// Load AchievementInfo starter data from achievement_info.json
pub async fn load_achievement_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/achievement_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_achievement_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    let array = match data.get("achievementInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!(
                "load_achievement_info: invalid or missing 'achievementInfo' array in achievement_info.json"
            );
            return Ok(());
        }
    };

    for entry in array {
        sqlx::query(
            r#"INSERT INTO AchievementInfo (
                Uid, GroupId, Value, MaxClearId, ContentsGroup            ) VALUES (
                ?, ?, ?, ?, ?            )"#,
        )
        .bind(uid)
        .bind(entry.get("groupId").and_then(|v| v.as_i64()))
        .bind(entry.get("value").and_then(|v| v.as_i64()))
        .bind(entry.get("maxClearId").and_then(|v| v.as_i64()))
        .bind(entry.get("contentsGroup").and_then(|v| v.as_i64()))
        .execute(&mut **tx) // Changed from pool to tx
        .await?;
    }

    Ok(())
}

async fn load_cost_time_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/charge_cost_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_cost_time_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    let array = match data.get("costTimeInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("load_cost_time_info: invalid or missing 'costTimeInfo' array");
            return Ok(());
        }
    };

    // iterate costTimeInfo entries
    for entry in array {
        // --- Insert nested CostItemInfo first ---
        let mut cost_item_info_index: Option<i64> = None;

        if let Some(cost_item_info) = entry.get("costItemInfo").and_then(|v| v.as_object()) {
            let res = sqlx::query(
                r#"
                INSERT INTO CostItemInfo (
                    Uid, InvenIndex, Id, Type, Count, KeepFlag, TimeValue,
                    PictorialbookInfoIndex, ExpiryTime, SortId, UseCount
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(uid)
            .bind(cost_item_info.get("invenIndex").and_then(|v| v.as_i64()))
            .bind(cost_item_info.get("id").and_then(|v| v.as_i64()))
            .bind(cost_item_info.get("type").and_then(|v| v.as_i64()))
            .bind(cost_item_info.get("count").and_then(|v| v.as_i64()))
            .bind(cost_item_info.get("keepFlag").and_then(|v| v.as_i64()))
            .bind(cost_item_info.get("timeValue").and_then(|v| v.as_i64()))
            .bind(None::<i64>) // skip PictorialbookInfoIndex for now
            .bind(cost_item_info.get("expiryTime").and_then(|v| v.as_i64()))
            .bind(cost_item_info.get("sortId").and_then(|v| v.as_i64()))
            .bind(cost_item_info.get("useCount").and_then(|v| v.as_i64()))
            .execute(&mut **tx)
            .await?;

            cost_item_info_index = Some(res.last_insert_rowid());
        }

        // --- Insert CostTimeInfo referencing CostItemInfo.Index ---
        let last_charge_time = entry
            .get("lastChargeTime")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        let res = sqlx::query(
            r#"
            INSERT INTO CostTimeInfo (
                Uid, LastChargeTime, CostItemInfoIndex
            )
            VALUES (?, ?, ?)
            "#,
        )
        .bind(uid)
        .bind(last_charge_time)
        .bind(cost_item_info_index)
        .execute(&mut **tx)
        .await?;

        let cost_time_info_pk = res.last_insert_rowid();

        // --- Insert ChargeCostInfo referencing CostTimeInfo ---
        sqlx::query(
            r#"
            INSERT INTO ChargeCostInfo (
                Uid, CostTimeInfoIndex, EventScheduleInfoIndex
            )
            VALUES (?, ?, NULL)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(uid)
        .bind(cost_time_info_pk)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

/// Load ItemInfo nested starter data (called by parent)
pub async fn load_item_info_nested(
    tx: &mut Transaction<'_, Sqlite>, // Changed from pool to tx
    uid: i64,
    entry: &Value,
    field_name: &str,
) -> sqlx::Result<Option<i64>> {
    let nested = match entry.get(field_name) {
        Some(v) if !v.is_null() => v,
        _ => return Ok(None),
    };

    let items: Vec<&Value> = if let Some(arr) = nested.as_array() {
        arr.iter().collect()
    } else if nested.is_object() {
        vec![nested]
    } else {
        eprintln!("load_item_info_nested: {field_name} is neither array nor object");
        return Ok(None);
    };

    let mut first_id = None;

    for item in items {
        let nested_obj = match item.as_object() {
            Some(o) => o,
            None => {
                eprintln!("load_item_info_nested: item in {field_name} is not an object");
                continue;
            }
        };

        let inven_index = nested_obj
            .get("invenIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let id = nested_obj
            .get("id")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let r#type = nested_obj
            .get("type")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let count = nested_obj
            .get("count")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let keep_flag = nested_obj
            .get("keepFlag")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let res = sqlx::query(
            r#"
            INSERT INTO ItemInfo (Uid, InvenIndex, Id, Type, Count, KeepFlag)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(uid)
        .bind(inven_index)
        .bind(id)
        .bind(r#type)
        .bind(count)
        .bind(keep_flag)
        .execute(&mut **tx) // Changed from pool to tx
        .await?;

        if first_id.is_none() {
            first_id = Some(res.last_insert_rowid());
        }
    }

    Ok(first_id)
}

/// Load MonsterInfo starter data from monster_info.json
pub async fn load_monster_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/monster_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_monster_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    let arr = if let Some(a) = data.as_array() {
        a
    } else if let Some(a) = data.get("monsterInfo").and_then(|v| v.as_array()) {
        a
    } else {
        eprintln!("insert_monster_info: missing or invalid 'monsterInfo' array");
        return Ok(());
    };

    for entry in arr {
        let monster_id = entry
            .get("monsterId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let battle_deck = entry
            .get("battleDeck")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let respawn_time = entry
            .get("respawnTime")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let life_end_time = entry
            .get("lifeEndTime")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let group_id = entry
            .get("groupId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let active_flag = entry
            .get("activeFlag")
            .and_then(|v| v.as_bool())
            .unwrap_or(false) as i32;

        sqlx::query(
            r#"
INSERT INTO MonsterInfo (
    Uid,
    MonsterId,
    BattleDeck,
    RespawnTime,
    LifeEndTime,
    GroupId,
    ActiveFlag
) VALUES (?, ?, ?, ?, ?, ?, ?)
"#,
        )
        .bind(uid)
        .bind(monster_id)
        .bind(battle_deck)
        .bind(respawn_time)
        .bind(life_end_time)
        .bind(group_id)
        .bind(active_flag)
        .execute(&mut **tx) // Changed from pool to tx
        .await?;
    }

    Ok(())
}

/// Load PassInfo starter data from pass_info.json
pub async fn load_pass_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/pass_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_pass_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    let arr = match data.get("passInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_pass_info: missing or invalid 'passInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let id = entry.get("id").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
        let exp = entry
            .get("exp")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let active_premium_1 = entry
            .get("activePremium1")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO PassInfo (
    Uid,
    Id,
    Exp,
    ActivePremium1
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&id)
        .bind(&exp)
        .bind(&active_premium_1)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

/// Load ProductInfo starter data from cash_shop_info.json
pub async fn load_product_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/cash_shop_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_product_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    let arr = match data.get("productInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_product_info: missing or invalid 'productInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let id = entry.get("id").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
        let buy_count = entry
            .get("buyCount")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO ProductInfo (
    Uid,
    Id,
    BuyCount
) VALUES (
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&id)
        .bind(&buy_count)
        .execute(&mut **tx) // Changed from pool to tx
        .await?;
    }

    Ok(())
}

/// Load ScheduleInfo starter data and related SeasonInfo entries.
pub async fn load_schedule_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/schedule_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_schedule_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    let arr = match data.get("scheduleInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("load_schedule_info: missing or invalid 'scheduleInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let content_id = entry
            .get("contentId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        // ---------- Insert currentSeason ----------
        let current = entry.get("currentSeason").and_then(|v| v.as_object());
        let current_index = if let Some(season) = current {
            let res = sqlx::query(
                r#"
                INSERT INTO SeasonInfo (
                    Uid, Season, StartTime, EndTime, ErrorFlag, ReturnFlag, RankRewardGroupId
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(uid)
            .bind(season.get("season").and_then(|v| v.as_i64()))
            .bind(season.get("startTime").and_then(|v| v.as_i64()))
            .bind(season.get("endTime").and_then(|v| v.as_i64()))
            .bind(
                season
                    .get("errorFlag")
                    .and_then(|v| v.as_bool())
                    .map(|b| b as i32),
            )
            .bind(
                season
                    .get("returnFlag")
                    .and_then(|v| v.as_bool())
                    .map(|b| b as i32),
            )
            .bind(season.get("rankRewardGroupId").and_then(|v| v.as_i64()))
            .execute(&mut **tx)
            .await?;
            Some(res.last_insert_rowid())
        } else {
            None
        };

        // ---------- Insert nextSeason ----------
        let next = entry.get("nextSeason").and_then(|v| v.as_object());
        let next_index = if let Some(season) = next {
            let res = sqlx::query(
                r#"
                INSERT INTO SeasonInfo (
                    Uid, Season, StartTime, EndTime, ErrorFlag, ReturnFlag, RankRewardGroupId
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(uid)
            .bind(season.get("season").and_then(|v| v.as_i64()))
            .bind(season.get("startTime").and_then(|v| v.as_i64()))
            .bind(season.get("endTime").and_then(|v| v.as_i64()))
            .bind(
                season
                    .get("errorFlag")
                    .and_then(|v| v.as_bool())
                    .map(|b| b as i32),
            )
            .bind(
                season
                    .get("returnFlag")
                    .and_then(|v| v.as_bool())
                    .map(|b| b as i32),
            )
            .bind(season.get("rankRewardGroupId").and_then(|v| v.as_i64()))
            .execute(&mut **tx)
            .await?;
            Some(res.last_insert_rowid())
        } else {
            None
        };

        // ---------- Insert ScheduleInfo referencing those two SeasonInfo PKs ----------
        sqlx::query(
            r#"
            INSERT INTO ScheduleInfo (
                Uid, ContentId, CurrentSeasonIndex, NextSeasonIndex
            ) VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(uid)
        .bind(content_id)
        .bind(current_index)
        .bind(next_index)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

// Placeholder stubs for functions not shown in your code
// You'll need to refactor these similarly
pub async fn load_deck_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/deck_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_deck_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    // Extract array from nested field "deckInfo"
    let array = match data.get("deckInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("load_deck_info: invalid or missing 'deckInfo' array in deck_info.json");
            return Ok(());
        }
    };

    for entry in array {
        sqlx::query(
            r#"INSERT INTO DeckInfo (
                Uid, CharInvenIndex, Position, Sequence            ) VALUES (
                ?, ?, ?, ?            )"#,
        )
        .bind(uid)
        .bind(entry.get("charInvenIndex").and_then(|v| v.as_i64()))
        .bind(entry.get("position").and_then(|v| v.as_i64()))
        .bind(entry.get("sequence").and_then(|v| v.as_i64()))
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

async fn load_event_schedule_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/event_schedule_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_event_schedule_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    // Extract array from nested field "eventScheduleInfo"
    let arr = match data.get("eventScheduleInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_event_schedule_info: missing or invalid 'eventScheduleInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let id = entry.get("id").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
        let event_type = entry
            .get("eventType")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let event_id = entry
            .get("eventId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let event_sub_id = entry
            .get("eventSubId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let start_date = entry
            .get("startDate")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let end_date = entry
            .get("endDate")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let is_active = entry
            .get("isActive")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO EventScheduleInfo (
    Uid,
    Id,
    EventType,
    EventId,
    EventSubId,
    StartDate,
    EndDate,
    IsActive
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&id)
        .bind(&event_type)
        .bind(&event_id)
        .bind(&event_sub_id)
        .bind(&start_date)
        .bind(&end_date)
        .bind(&is_active)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn load_field_deck_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/field_deck_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_field_deck_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    // Extract array from nested field "fieldDeckInfo"
    let array = match data.get("fieldDeckInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!(
                "load_field_deck_info: invalid or missing 'fieldDeckInfo' array in field_deck_info.json"
            );
            return Ok(());
        }
    };

    for entry in array {
        sqlx::query(
            r#"INSERT INTO FieldDeckInfo (
                    Uid, Sequence, CharInvenIndex, CostumeInvenIndex            ) VALUES (
                    ?, ?, ?, ?            )"#,
        )
        .bind(uid)
        .bind(entry.get("sequence").and_then(|v| v.as_i64()))
        .bind(entry.get("charInvenIndex").and_then(|v| v.as_i64()))
        .bind(entry.get("costumeInvenIndex").and_then(|v| v.as_i64()))
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn load_hunting_ground_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/hunting_ground_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_hunting_ground_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    let array = data
        .get("huntingGroundInfo")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            eprintln!("load_hunting_ground_info: missing or invalid 'huntingGroundInfo' array");
            sqlx::Error::Protocol("invalid JSON".into())
        })?;

    for entry in array {
        let mut monster_indices = Vec::new();

        if let Some(monsters) = entry.get("monsterInfo").and_then(|v| v.as_array()) {
            for monster in monsters {
                let res = sqlx::query(
                    r#"
                    INSERT INTO MonsterInfo
                        (Uid, MonsterId, BattleDeck, RespawnTime, LifeEndTime, GroupId, ActiveFlag)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(uid)
                .bind(monster.get("monsterId").and_then(|v| v.as_i64()))
                .bind(monster.get("battleDeck").and_then(|v| v.as_i64()))
                .bind(monster.get("respawnTime").and_then(|v| v.as_i64()))
                .bind(monster.get("lifeEndTime").and_then(|v| v.as_i64()))
                .bind(monster.get("groupId").and_then(|v| v.as_i64()))
                .bind(
                    monster
                        .get("activeFlag")
                        .and_then(|v| v.as_bool())
                        .map(|b| b as i32),
                )
                .execute(&mut **tx)
                .await?;

                monster_indices.push(res.last_insert_rowid());
            }
        }

        let res = sqlx::query(
            r#"
            INSERT INTO HuntingGroundInfo
                (Uid, IsAuto, CurrentId, HighestId, PackId)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(uid)
        .bind(
            entry
                .get("isAuto")
                .and_then(|v| v.as_bool())
                .map(|b| b as i32),
        )
        .bind(entry.get("currentId").and_then(|v| v.as_i64()))
        .bind(entry.get("highestId").and_then(|v| v.as_i64()))
        .bind(entry.get("packId").and_then(|v| v.as_i64()))
        .execute(&mut **tx)
        .await?;

        let hunting_index = res.last_insert_rowid();

        for monster_idx in monster_indices {
            sqlx::query(
                r#"
                INSERT INTO HuntingGroundMonster (HuntingGroundIndex, MonsterIndex)
                VALUES (?, ?)
                "#,
            )
            .bind(hunting_index)
            .bind(monster_idx)
            .execute(&mut **tx)
            .await?;
        }
    }

    Ok(())
}

async fn load_item_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/item_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_item_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    // Extract array from nested field "itemInfo"
    let array = match data.get("itemInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("load_item_info: invalid or missing 'itemInfo' array in item_info.json");
            return Ok(());
        }
    };

    for entry in array {
        // handle nested PictorialBookInfo
        let pictorialbook_info_index =
            load_pictorial_book_info_nested(tx, uid, entry, "pictorialbookInfo")
                .await?
                .unwrap_or_default();

        sqlx::query(
                r#"INSERT INTO ItemInfo (
                    Uid, InvenIndex, Id, Type, Count, KeepFlag, TimeValue, ExpiryTime, SortId, UseCount, PictorialbookInfoIndex            ) VALUES (
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?            )"#
            )
            .bind(uid)
            .bind(entry.get("invenIndex").and_then(|v| v.as_i64()))
            .bind(entry.get("id").and_then(|v| v.as_i64()))
            .bind(entry.get("type").and_then(|v| v.as_i64()))
            .bind(entry.get("count").and_then(|v| v.as_i64()))
            .bind(entry.get("keepFlag").and_then(|v| v.as_i64()))
            .bind(entry.get("timeValue").and_then(|v| v.as_i64()))
            .bind(entry.get("expiryTime").and_then(|v| v.as_i64()))
            .bind(entry.get("sortId").and_then(|v| v.as_i64()))
            .bind(entry.get("useCount").and_then(|v| v.as_i64()))
            .bind(pictorialbook_info_index)
            .execute(&mut **tx)
            .await?;
    }
    Ok(())
}

pub async fn load_pictorial_book_info_nested(
    tx: &mut Transaction<'_, Sqlite>,
    uid: i64,
    entry: &Value,
    field_name: &str,
) -> sqlx::Result<Option<i64>> {
    let nested = match entry.get(field_name) {
        Some(v) if !v.is_null() => v,
        _ => return Ok(None),
    };

    // Handle both array and single object cases
    let items: Vec<&Value> = if let Some(arr) = nested.as_array() {
        arr.iter().collect()
    } else if nested.is_object() {
        vec![nested]
    } else {
        eprintln!("load_pictorial_book_info_nested: {field_name} is neither array nor object");
        return Ok(None);
    };

    let mut first_id = None;

    for item in items {
        let nested_obj = match item.as_object() {
            Some(o) => o,
            None => {
                eprintln!("load_pictorial_book_info_nested: item in {field_name} is not an object");
                continue;
            }
        };

        let id = nested_obj
            .get("id")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let group_id = nested_obj
            .get("groupId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        let result = sqlx::query(
            r#"INSERT INTO PictorialBookInfo (
                Uid, Id, GroupId            ) VALUES (
                ?, ?, ?            )"#,
        )
        .bind(uid)
        .bind(id)
        .bind(group_id)
        .execute(&mut **tx)
        .await?;

        // Store the first inserted rowid
        if first_id.is_none() {
            first_id = Some(result.last_insert_rowid());
        }
    }

    Ok(first_id)
}

async fn load_mail_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/mail_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_mail_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    // Extract array from nested field "mailInfo"
    let arr = match data.get("mailInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_mail_info: missing or invalid 'mailInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle parallel repeated arrays - iterate all together
        let inven_index = entry
            .get("invenIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let r#type = entry
            .get("type")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let mail_id = entry
            .get("mailId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let sender_text = entry
            .get("senderText")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let title_text = entry
            .get("titleText")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let message_text = entry
            .get("messageText")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let reward_expire_time = entry
            .get("rewardExpireTime")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let is_open = entry
            .get("isOpen")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let open_time = entry
            .get("openTime")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let create_time = entry
            .get("createTime")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let history_delete_time = entry
            .get("historyDeleteTime")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let is_cash = entry
            .get("isCash")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        // Get all parallel arrays
        let item_type_array = entry.get("itemType").and_then(|v| v.as_array());
        let item_id_array = entry.get("itemId").and_then(|v| v.as_array());
        let item_count_array = entry.get("itemCount").and_then(|v| v.as_array());

        // Determine max length
        let len = 0
            .max(item_type_array.map(|a| a.len()).unwrap_or(0))
            .max(item_id_array.map(|a| a.len()).unwrap_or(0))
            .max(item_count_array.map(|a| a.len()).unwrap_or(0));

        if len > 0 {
            // Insert one row per index (parallel iteration)
            for i in 0..len {
                let item_type = item_type_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                let item_id = item_id_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                let item_count = item_count_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;

                sqlx::query(
                    r#"
    INSERT INTO MailInfo (
        Uid,
        InvenIndex,
        Type,
        MailId,
        SenderText,
        TitleText,
        MessageText,
        RewardExpireTime,
        ItemType,
        ItemId,
        ItemCount,
        IsOpen,
        OpenTime,
        CreateTime,
        HistoryDeleteTime,
        IsCash
    ) VALUES (
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?
    )
    "#,
                )
                .bind(uid)
                .bind(&inven_index)
                .bind(&r#type)
                .bind(&mail_id)
                .bind(&sender_text)
                .bind(&title_text)
                .bind(&message_text)
                .bind(&reward_expire_time)
                .bind(item_type)
                .bind(item_id)
                .bind(item_count)
                .bind(&is_open)
                .bind(&open_time)
                .bind(&create_time)
                .bind(&history_delete_time)
                .bind(&is_cash)
                .execute(&mut **tx)
                .await?;
            }
        } else {
            // No items, insert one row with NULLs for repeated fields
            sqlx::query(
                r#"
    INSERT INTO MailInfo (
        Uid,
        InvenIndex,
        Type,
        MailId,
        SenderText,
        TitleText,
        MessageText,
        RewardExpireTime,
        ItemType,
        ItemId,
        ItemCount,
        IsOpen,
        OpenTime,
        CreateTime,
        HistoryDeleteTime,
        IsCash
    ) VALUES (
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        NULL,
        NULL,
        NULL,
        ?,
        ?,
        ?,
        ?,
        ?
    )
    "#,
            )
            .bind(uid)
            .bind(&inven_index)
            .bind(&r#type)
            .bind(&mail_id)
            .bind(&sender_text)
            .bind(&title_text)
            .bind(&message_text)
            .bind(&reward_expire_time)
            .bind(&is_open)
            .bind(&open_time)
            .bind(&create_time)
            .bind(&history_delete_time)
            .bind(&is_cash)
            .execute(&mut **tx)
            .await?;
        }
    }

    Ok(())
}

async fn load_mission_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let json_str = include_str!("../../../../data/starter/event_mission_info.json");
    let data: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("load_mission_info: failed to parse JSON: {e}");
            return Ok(());
        }
    };

    // Extract array from nested field "missionInfo"
    let arr = match data.get("missionInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_mission_info: missing or invalid 'missionInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let group_id = entry
            .get("groupId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let id = entry.get("id").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
        let group_type = entry
            .get("groupType")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let value = entry
            .get("value")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let is_complete = entry
            .get("isComplete")
            .and_then(|v| v.as_bool())
            .unwrap_or_default() as bool;

        sqlx::query(
            r#"
    INSERT INTO MissionInfo (
        Uid,
        GroupId,
        Id,
        GroupType,
        Value,
        IsComplete
    ) VALUES (
        ?,
        ?,
        ?,
        ?,
        ?,
        ?
    )
    "#,
        )
        .bind(uid)
        .bind(&group_id)
        .bind(&id)
        .bind(&group_type)
        .bind(&value)
        .bind(&is_complete)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

// ========================================
// Master Loader for All Starter Tables
// ========================================
pub async fn load_all_starter_data(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    tracing::info!("Loading all starter data for uid {uid} in a single transaction");

    // Begin a single transaction for all operations
    let mut tx = pool.begin().await?;

    // Call all loader functions with the transaction
    load_char_info(&mut tx, uid).await?;
    load_costume_info(&mut tx, uid).await?;
    load_achievement_info(&mut tx, uid).await?;
    load_cost_time_info(&mut tx, uid).await?;
    load_deck_info(&mut tx, uid).await?;
    load_event_schedule_info(&mut tx, uid).await?;
    load_field_deck_info(&mut tx, uid).await?;
    load_hunting_ground_info(&mut tx, uid).await?;
    load_item_info(&mut tx, uid).await?;
    load_mail_info(&mut tx, uid).await?;
    load_mission_info(&mut tx, uid).await?;
    load_monster_info(&mut tx, uid).await?;
    load_pass_info(&mut tx, uid).await?;
    load_product_info(&mut tx, uid).await?;
    load_schedule_info(&mut tx, uid).await?;

    // Commit the transaction - all or nothing!
    tx.commit().await?;

    tracing::info!("Finished loading all starter data for uid {uid}");
    Ok(())
}
