// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Squaredefaulttable {
    #[serde(rename = "avatarSetClearContentTicketId", default)]
    pub avatar_set_clear_content_ticket_id: Vec<i32>,
    #[serde(rename = "defaultChannelCount", default)]
    pub default_channel_count: i32,
    #[serde(rename = "defaultSpawnPostion", default)]
    pub default_spawn_postion: Vec<i32>,
    #[serde(rename = "limitChannelUserCount", default)]
    pub limit_channel_user_count: i32,
    #[serde(rename = "newChannelDesnsityValue", default)]
    pub new_channel_desnsity_value: i32,
    #[serde(rename = "questNameTextId", default)]
    pub quest_name_text_id: i32,
    #[serde(rename = "questSkipTextId", default)]
    pub quest_skip_text_id: i32,
    #[serde(rename = "questTitleQuestTextId", default)]
    pub quest_title_quest_text_id: i32,
    #[serde(rename = "squareTimelineName", default)]
    pub square_timeline_name: String,
    #[serde(rename = "squareTutorialId", default)]
    pub square_tutorial_id: i32,
    #[serde(rename = "squareVisualNovelDialogId", default)]
    pub square_visual_novel_dialog_id: i32,
}

pub struct SquaredefaulttableTable {
    records: Vec<Squaredefaulttable>,
}

impl SquaredefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Squaredefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Squaredefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Squaredefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
