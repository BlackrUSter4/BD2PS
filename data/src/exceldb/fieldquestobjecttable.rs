// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldquestobjecttable {
    #[serde(rename = "animatorType", default)]
    pub animator_type: Option<i32>,
    #[serde(rename = "bundleType", default)]
    pub bundle_type: Option<i32>,
    #[serde(rename = "conditionType", default)]
    pub condition_type: i32,
    #[serde(rename = "directionType", default)]
    pub direction_type: Option<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "interactionLocalTextId", default)]
    pub interaction_local_text_id: Option<i32>,
    #[serde(rename = "isAutoInteraction", default)]
    pub is_auto_interaction: Option<i32>,
    #[serde(rename = "isDisableInteraction", default)]
    pub is_disable_interaction: Option<i32>,
    #[serde(rename = "isDisableQuestMark", default)]
    pub is_disable_quest_mark: Option<i32>,
    #[serde(rename = "isDisplayMiniMap", default)]
    pub is_display_mini_map: Option<i32>,
    #[serde(rename = "markerRadius", default)]
    pub marker_radius: Option<f32>,
    #[serde(rename = "questRange", default)]
    pub quest_range: Vec<i32>,
    #[serde(rename = "questSimpleTalkGroupId", default)]
    pub quest_simple_talk_group_id: Option<i32>,
    #[serde(rename = "rangeType", default)]
    pub range_type: Option<i32>,
    #[serde(rename = "resourceName", default)]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType", default)]
    pub resource_type: Option<i32>,
    #[serde(rename = "rewardCount", default)]
    pub reward_count: Option<i32>,
    #[serde(rename = "rewardId", default)]
    pub reward_id: Option<i32>,
    #[serde(rename = "rewardType", default)]
    pub reward_type: Option<i32>,
    #[serde(rename = "specialAnimationState", default)]
    pub special_animation_state: Option<i32>,
    /// Not a real client-sent field; synthesized at import time from which
    /// server/Data/PACK/<n>/ folder a row came from, since `id` is only
    /// unique within a single pack, not globally. See CLIENT_UPDATE.md's
    /// 2026-10-02 "(packId, id) composite key" entries.
    #[serde(rename = "PackId", default)]
    pub pack_id: i32,
}

pub struct FieldquestobjecttableTable {
    records: Vec<Fieldquestobjecttable>,
    by_id: HashMap<i32, usize>,
    by_pack: HashMap<(i32, i32), usize>,
}

impl FieldquestobjecttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldquestobjecttable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_pack = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_pack.insert((record.pack_id, record.id), idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_pack,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Fieldquestobjecttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    /// Pack-scoped lookup — use this over `get()` for any new call site,
    /// since `id` collides across packs (see `pack_id` field doc).
    #[inline]
    pub fn get_by_pack(&self, pack_id: i32, id: i32) -> Option<&Fieldquestobjecttable> {
        self.by_pack.get(&(pack_id, id)).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldquestobjecttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldquestobjecttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
