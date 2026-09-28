use crate::models::game::cash::cash_bonus_info::CashBonusInfo;
use sqlx::SqlitePool;

pub async fn get_all(pool: &SqlitePool, uid: i64) -> Vec<CashBonusInfo> {
    sqlx::query_as::<_, CashBonusInfo>("SELECT * FROM CashBonusInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
}

pub async fn get_one(pool: &SqlitePool, uid: i64, group_id: i32, contents_group_id: i32) -> CashBonusInfo {
    sqlx::query_as::<_, CashBonusInfo>(
        "SELECT * FROM CashBonusInfo WHERE Uid = ? AND GroupId = ? AND ContentsGroupId = ?",
    )
    .bind(uid)
    .bind(group_id)
    .bind(contents_group_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
    .unwrap_or(CashBonusInfo {
        uid,
        group_id,
        contents_group_id,
        buy_count: 0,
        rewarded_ids: None,
    })
}

pub async fn save(pool: &SqlitePool, info: &CashBonusInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CashBonusInfo (Uid, GroupId, ContentsGroupId, BuyCount, RewardedIds)
VALUES (?, ?, ?, ?, ?)
ON CONFLICT(Uid, GroupId, ContentsGroupId) DO UPDATE SET
    BuyCount = excluded.BuyCount,
    RewardedIds = excluded.RewardedIds
"#,
    )
    .bind(info.uid)
    .bind(info.group_id)
    .bind(info.contents_group_id)
    .bind(info.buy_count)
    .bind(&info.rewarded_ids)
    .execute(pool)
    .await?;
    Ok(())
}
