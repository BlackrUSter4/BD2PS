// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cafeteriadefaulttable {
    #[serde(rename = "NpcSpawnDistance", default)]
    pub npc_spawn_distance: i32,
    #[serde(rename = "basicInteractionMaxCount", default)]
    pub basic_interaction_max_count: i32,
    #[serde(rename = "basicInteractionTerm", default)]
    pub basic_interaction_term: i32,
    #[serde(rename = "bubbleDuration", default)]
    pub bubble_duration: i32,
    #[serde(rename = "bubbleMaxCount", default)]
    pub bubble_max_count: i32,
    #[serde(rename = "cafeteriaGuideId", default)]
    pub cafeteria_guide_id: i32,
    #[serde(rename = "cafeteriaNoteConditionValue", default)]
    pub cafeteria_note_condition_value: i32,
    #[serde(rename = "dailyShopCurrencyLimit", default)]
    pub daily_shop_currency_limit: i32,
    #[serde(rename = "defaultBasicInterationCount", default)]
    pub default_basic_interation_count: i32,
    #[serde(rename = "defaultUniqueInterationCount", default)]
    pub default_unique_interation_count: i32,
    #[serde(rename = "eventRewardType", default)]
    pub event_reward_type: i32,
    #[serde(rename = "interactionIgonoreTime", default)]
    pub interaction_igonore_time: i32,
    #[serde(rename = "lowMemSpawnCount", default)]
    pub low_mem_spawn_count: i32,
    #[serde(rename = "maxRewardTime", default)]
    pub max_reward_time: i32,
    #[serde(rename = "minRewardTime", default)]
    pub min_reward_time: i32,
    #[serde(rename = "normalMemSpawnCount", default)]
    pub normal_mem_spawn_count: i32,
    #[serde(rename = "noteRewardType", default)]
    pub note_reward_type: i32,
    #[serde(rename = "noteRewardValue", default)]
    pub note_reward_value: i32,
    #[serde(rename = "questNameTextId", default)]
    pub quest_name_text_id: i32,
    #[serde(rename = "questSkipTextId", default)]
    pub quest_skip_text_id: i32,
    #[serde(rename = "startTimelineName", default)]
    pub start_timeline_name: String,
    #[serde(rename = "startVisualNovelDialogId", default)]
    pub start_visual_novel_dialog_id: i32,
    #[serde(rename = "uniqueInteractionMaxCount", default)]
    pub unique_interaction_max_count: i32,
    #[serde(rename = "uniqueInteractionTerm", default)]
    pub unique_interaction_term: i32,
    #[serde(rename = "visualNovelEndRewardCount", default)]
    pub visual_novel_end_reward_count: i32,
    #[serde(rename = "visualNovelEndRewardId", default)]
    pub visual_novel_end_reward_id: i32,
    #[serde(rename = "visualNovelEndRewardType", default)]
    pub visual_novel_end_reward_type: i32,
    #[serde(rename = "questTitleQuestTextId", default)]
    pub quest_title_quest_text_id: Option<i32>,
}

pub struct CafeteriadefaulttableTable {
    records: Vec<Cafeteriadefaulttable>,
}

impl CafeteriadefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cafeteriadefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Cafeteriadefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Cafeteriadefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
