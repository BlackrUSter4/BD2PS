use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CharAutoReviveSetting {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "CanAutoRevive")]
    pub can_auto_revive: bool,
    #[sqlx(rename = "CastingCharInvenIndex")]
    pub casting_char_inven_index: Option<i64>,
}
