use database::{db, models::game::user::user_info::UserInfo, models::user::account::Account};
use sqlx::SqlitePool;
use tracing::info;

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("Invalid access token")]
    InvalidToken,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

/// Parse UID from access token (format: "uid|other_data")
pub fn parse_uid_from_token(token: &str) -> Result<i64, LoginError> {
    token
        .split('|')
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or(LoginError::InvalidToken)
}

/// Get or create user, returning both account and user info
pub async fn get_or_create_user(
    pool: &SqlitePool,
    uid: i64,
) -> Result<(Account, UserInfo), LoginError> {
    // Use the database layer's get_or_create_user which handles the mapping
    let (account, user_info) = db::user::user::get_or_create_user(pool, uid).await?;

    info!("User logged in: {}", account.user_name);
    Ok((account, user_info))
}

/// Update user's return status timestamp
pub async fn update_login_timestamp(pool: &SqlitePool, uid: i64) -> Result<(), LoginError> {
    let now_ms = chrono::Utc::now().timestamp_millis();
    db::user::user::update_return_status(pool, uid, now_ms).await?;
    info!("Updated return status for UID {} to {}", uid, now_ms);
    Ok(())
}

pub async fn get_owner_index_for_uid(pool: &SqlitePool, uid: i64) -> Result<i64, LoginError> {
    let owner_index = sqlx::query_scalar::<_, i64>("SELECT OwnerIndex FROM UserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;

    Ok(owner_index)
}

/// Get UID for a given owner_index from UserMapper table
pub async fn get_uid_for_owner_index(
    pool: &SqlitePool,
    owner_index: i64,
) -> Result<i64, LoginError> {
    let uid = sqlx::query_scalar::<_, i64>("SELECT Uid FROM UserInfo WHERE OwnerIndex = ?")
        .bind(owner_index)
        .fetch_one(pool)
        .await?;

    Ok(uid)
}
