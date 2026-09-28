// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cafeteriadefaulttable {
    #[serde(rename = "NpcSpawnDistance")]
    pub npc_spawn_distance: i32,
    #[serde(rename = "basicInteractionMaxCount")]
    pub basic_interaction_max_count: i32,
    #[serde(rename = "basicInteractionTerm")]
    pub basic_interaction_term: i32,
    #[serde(rename = "bubbleDuration")]
    pub bubble_duration: i32,
    #[serde(rename = "bubbleMaxCount")]
    pub bubble_max_count: i32,
    #[serde(rename = "cafeteriaGuideId")]
    pub cafeteria_guide_id: i32,
    #[serde(rename = "cafeteriaNoteConditionValue")]
    pub cafeteria_note_condition_value: i32,
    #[serde(rename = "dailyShopCurrencyLimit")]
    pub daily_shop_currency_limit: i32,
    #[serde(rename = "defaultBasicInterationCount")]
    pub default_basic_interation_count: i32,
    #[serde(rename = "defaultUniqueInterationCount")]
    pub default_unique_interation_count: i32,
    #[serde(rename = "eventRewardType")]
    pub event_reward_type: i32,
    #[serde(rename = "interactionIgonoreTime")]
    pub interaction_igonore_time: i32,
    #[serde(rename = "lowMemSpawnCount")]
    pub low_mem_spawn_count: i32,
    #[serde(rename = "maxRewardTime")]
    pub max_reward_time: i32,
    #[serde(rename = "minRewardTime")]
    pub min_reward_time: i32,
    #[serde(rename = "normalMemSpawnCount")]
    pub normal_mem_spawn_count: i32,
    #[serde(rename = "noteRewardType")]
    pub note_reward_type: i32,
    #[serde(rename = "noteRewardValue")]
    pub note_reward_value: i32,
    #[serde(rename = "questNameTextId")]
    pub quest_name_text_id: i32,
    #[serde(rename = "questSkipTextId")]
    pub quest_skip_text_id: i32,
    #[serde(rename = "startTimelineName")]
    pub start_timeline_name: String,
    #[serde(rename = "startVisualNovelDialogId")]
    pub start_visual_novel_dialog_id: i32,
    #[serde(rename = "uniqueInteractionMaxCount")]
    pub unique_interaction_max_count: i32,
    #[serde(rename = "uniqueInteractionTerm")]
    pub unique_interaction_term: i32,
    #[serde(rename = "visualNovelEndRewardCount")]
    pub visual_novel_end_reward_count: i32,
    #[serde(rename = "visualNovelEndRewardId")]
    pub visual_novel_end_reward_id: i32,
    #[serde(rename = "visualNovelEndRewardType")]
    pub visual_novel_end_reward_type: i32,
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
    pub fn iter(&self) -> std::slice::Iter<Cafeteriadefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
