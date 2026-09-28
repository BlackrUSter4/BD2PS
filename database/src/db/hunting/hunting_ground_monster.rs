use crate::models::game::monster::monster_info::MonsterInfo;
use sqlx::SqlitePool;

pub async fn get_monsters_for_hunting_ground(
    pool: &SqlitePool,
    hunting_ground_index: i64,
) -> sqlx::Result<Vec<MonsterInfo>> {
    sqlx::query_as::<_, MonsterInfo>(
        r#"
        SELECT m.*
        FROM MonsterInfo m
        INNER JOIN HuntingGroundMonster hgm
            ON hgm.MonsterIndex = m."Index"
        WHERE hgm.HuntingGroundIndex = ?
        "#,
    )
    .bind(hunting_ground_index)
    .fetch_all(pool)
    .await
}
