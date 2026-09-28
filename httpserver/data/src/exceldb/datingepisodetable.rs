// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datingepisodetable {
    #[serde(rename = "datingDialogGroupId")]
    pub dating_dialog_group_id: i32,
    #[serde(rename = "datingMessage1")]
    pub dating_message1: Option<i32>,
    #[serde(rename = "datingMessage2")]
    pub dating_message2: i32,
    #[serde(rename = "episodeTitleDatingTextId")]
    pub episode_title_dating_text_id: i32,
    #[serde(rename = "episodeType")]
    pub episode_type: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "rewardCount")]
    pub reward_count: i32,
    #[serde(rename = "rewardDatingPoint")]
    pub reward_dating_point: i32,
    #[serde(rename = "rewardId")]
    pub reward_id: i32,
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
    #[serde(rename = "skipQuestTextId")]
    pub skip_quest_text_id: i32,
    #[serde(rename = "talkStatusTextId")]
    pub talk_status_text_id: i32,
    #[serde(rename = "timelineName")]
    pub timeline_name: String,
}

pub struct DatingepisodetableTable {
    records: Vec<Datingepisodetable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl DatingepisodetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Datingepisodetable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.dating_dialog_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Datingepisodetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Datingepisodetable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Datingepisodetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Datingepisodetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
