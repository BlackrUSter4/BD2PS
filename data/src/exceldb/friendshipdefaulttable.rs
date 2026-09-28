// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friendshipdefaulttable {
    #[serde(rename = "correctExp")]
    pub correct_exp: i32,
    #[serde(rename = "counselingCompleteRewardCount")]
    pub counseling_complete_reward_count: i32,
    #[serde(rename = "counselingCompleteRewardType")]
    pub counseling_complete_reward_type: i32,
    #[serde(rename = "eventMissionGroupId")]
    pub event_mission_group_id: i32,
    #[serde(rename = "friendshipMaxLevel1")]
    pub friendship_max_level1: i32,
    #[serde(rename = "friendshipMaxLevel2")]
    pub friendship_max_level2: i32,
    #[serde(rename = "friendshipMaxLevel3")]
    pub friendship_max_level3: i32,
    #[serde(rename = "guideGroupId")]
    pub guide_group_id: i32,
    #[serde(rename = "incorrectExp")]
    pub incorrect_exp: i32,
    #[serde(rename = "maxCounselingAP")]
    pub max_counseling_ap: i32,
    #[serde(rename = "maxCounselingAPByCostume")]
    pub max_counseling_ap_by_costume: i32,
    #[serde(rename = "questNameTextId")]
    pub quest_name_text_id: i32,
    #[serde(rename = "questSkipTextId")]
    pub quest_skip_text_id: i32,
    #[serde(rename = "questTitleQuestTextId")]
    pub quest_title_quest_text_id: i32,
    #[serde(rename = "quickCounselingUnlockCount")]
    pub quick_counseling_unlock_count: i32,
    #[serde(rename = "startTimelineName")]
    pub start_timeline_name: String,
    #[serde(rename = "startVisualNovelDialogId")]
    pub start_visual_novel_dialog_id: i32,
}

pub struct FriendshipdefaulttableTable {
    records: Vec<Friendshipdefaulttable>,
}

impl FriendshipdefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Friendshipdefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Friendshipdefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Friendshipdefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
