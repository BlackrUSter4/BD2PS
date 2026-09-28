use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct DatingInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EpisodeInfoIndex")]
    pub episode_info_index: Option<String>,
    #[sqlx(rename = "MessageChoiceInfoIndex")]
    pub message_choice_info_index: Option<String>,
}