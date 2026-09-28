// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldquestobjecttable {
    #[serde(rename = "animatorType")]
    pub animator_type: Option<i32>,
    #[serde(rename = "bundleType")]
    pub bundle_type: Option<i32>,
    #[serde(rename = "conditionType")]
    pub condition_type: i32,
    #[serde(rename = "directionType")]
    pub direction_type: Option<i32>,
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "interactionLocalTextId")]
    pub interaction_local_text_id: Option<i32>,
    #[serde(rename = "isAutoInteraction")]
    pub is_auto_interaction: Option<i32>,
    #[serde(rename = "isDisableInteraction")]
    pub is_disable_interaction: Option<i32>,
    #[serde(rename = "isDisableQuestMark")]
    pub is_disable_quest_mark: Option<i32>,
    #[serde(rename = "isDisplayMiniMap")]
    pub is_display_mini_map: Option<i32>,
    #[serde(rename = "markerRadius")]
    pub marker_radius: Option<f32>,
    #[serde(rename = "questRange")]
    pub quest_range: Vec<i32>,
    #[serde(rename = "questSimpleTalkGroupId")]
    pub quest_simple_talk_group_id: Option<i32>,
    #[serde(rename = "rangeType")]
    pub range_type: Option<i32>,
    #[serde(rename = "resourceName")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType")]
    pub resource_type: Option<i32>,
    #[serde(rename = "rewardCount")]
    pub reward_count: Option<i32>,
    #[serde(rename = "rewardId")]
    pub reward_id: Option<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: Option<i32>,
    #[serde(rename = "specialAnimationState")]
    pub special_animation_state: Option<i32>,
}

pub struct FieldquestobjecttableTable {
    records: Vec<Fieldquestobjecttable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl FieldquestobjecttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldquestobjecttable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            if let Some(id) = record.id {
                by_id.insert(id, idx);
            }
            if let Some(group_id) = record.quest_simple_talk_group_id {
                by_group.entry(group_id).or_insert_with(Vec::new).push(idx);
            }
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Fieldquestobjecttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Fieldquestobjecttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldquestobjecttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Fieldquestobjecttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
