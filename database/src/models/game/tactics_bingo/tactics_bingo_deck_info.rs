use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct TacticsBingoDeckInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    /// JSON-encoded Vec<(char_inven_index, char_id, costume_id, position, sequence)>.
    #[sqlx(rename = "DeckInfo")]
    pub deck_info: Option<String>,
}
