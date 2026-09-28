use crate::models::game::char::char_info::CharInfo;
use sqlx::SqlitePool;

/// Add a single CharInfo record from a Rust struct.
pub async fn add_char_info(pool: &SqlitePool, data: &CharInfo) -> sqlx::Result<()> {
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
    .bind(&data.uid)
    .bind(&data.inven_index)
    .bind(&data.id)
    .bind(&data.hp)
    .bind(&data.level)
    .bind(&data.costume_id)
    .bind(&data.exp)
    .bind(&data.use_costume)
    .bind(&data.talent_level)
    .bind(&data.talent_exp)
    .bind(&data.solidarity_reward)
    .bind(&data.expiry_time)
    .bind(&data.pictorialbook_info_index)
    .bind(&data.connect_potential_costume)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_char_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<CharInfo>> {
    sqlx::query_as::<_, CharInfo>("SELECT * FROM CharInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all CharInfo rows for a UID.
pub async fn delete_char_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CharInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<CharInfo> {
    sqlx::query_as::<_, CharInfo>("SELECT * FROM CharInfo WHERE Uid = ? AND Index = ?")
        .bind(uid)
        .bind(index)
        .fetch_one(pool)
        .await
}

/// Get multiple records by their Index (rowids)
pub async fn get_all_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: &[i64],
) -> sqlx::Result<Vec<CharInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CharInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CharInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CharInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
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
    .bind(&data.uid)
    .bind(&data.inven_index)
    .bind(&data.id)
    .bind(&data.hp)
    .bind(&data.level)
    .bind(&data.costume_id)
    .bind(&data.exp)
    .bind(&data.use_costume)
    .bind(&data.talent_level)
    .bind(&data.talent_exp)
    .bind(&data.solidarity_reward)
    .bind(&data.expiry_time)
    .bind(&data.pictorialbook_info_index)
    .bind(&data.connect_potential_costume)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn set_use_costume(
    pool: &SqlitePool,
    uid: i64,
    char_inven_index: i64,
    costume_inven_index: i64,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE CharInfo SET UseCostume = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(costume_inven_index)
        .bind(uid)
        .bind(char_inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_connect_potential_costume(
    pool: &SqlitePool,
    uid: i64,
    char_inven_index: i64,
    costume_id: i32,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE CharInfo SET ConnectPotentialCostume = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(costume_id)
        .bind(uid)
        .bind(char_inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_hp(pool: &SqlitePool, uid: i64, index: i64, hp: i64) -> sqlx::Result<()> {
    sqlx::query("UPDATE CharInfo SET Hp = ? WHERE Uid = ? AND Index = ?")
        .bind(hp)
        .bind(uid)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by InvenIndex.
pub async fn get_by_inven_index(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
) -> sqlx::Result<Option<CharInfo>> {
    sqlx::query_as::<_, CharInfo>("SELECT * FROM CharInfo WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .fetch_optional(pool)
        .await
}

/// Get multiple records by their InvenIndex values.
pub async fn get_all_by_inven_index(
    pool: &SqlitePool,
    uid: i64,
    inven_index: &[i64],
) -> sqlx::Result<Vec<CharInfo>> {
    if inven_index.is_empty() {
        return Ok(vec![]);
    }
    let placeholders = inven_index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CharInfo WHERE Uid = ? AND InvenIndex IN ({})",
        placeholders
    );
    let mut query = sqlx::query_as::<_, CharInfo>(&query).bind(uid);
    for idx in inven_index {
        query = query.bind(idx);
    }
    query.fetch_all(pool).await
}

/// Update level + exp for a character (level-up / growth feeding).
pub async fn set_level_exp(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    level: i32,
    exp: i32,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE CharInfo SET Level = ?, Exp = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(level)
        .bind(exp)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Bump the class/tier stage for a character (CharClassUp).
pub async fn set_class_level(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    class_level: i32,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE CharInfo SET ClassLevel = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(class_level)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Update Hp for a character looked up by InvenIndex (rather than Index).
pub async fn set_hp_by_inven(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    hp: i64,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE CharInfo SET Hp = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(hp)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete a single character row by its InvenIndex (e.g. CharExpiry cleanup).
pub async fn delete_by_inven_index(pool: &SqlitePool, uid: i64, inven_index: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CharInfo WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}
