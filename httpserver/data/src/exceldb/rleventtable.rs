// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rleventtable {
    #[serde(rename = "choice1EffectId")]
    pub choice1_effect_id: Vec<i32>,
    #[serde(rename = "choice1FailEffectId")]
    pub choice1_fail_effect_id: Vec<i32>,
    #[serde(rename = "choice2EffectId")]
    pub choice2_effect_id: Vec<i32>,
    #[serde(rename = "choiceDescNameTextId")]
    pub choice_desc_name_text_id: Vec<i32>,
    #[serde(rename = "choiceFailResultNameTextId")]
    pub choice_fail_result_name_text_id: Vec<i32>,
    #[serde(rename = "choiceResultNameTextId")]
    pub choice_result_name_text_id: Vec<i32>,
    #[serde(rename = "choiceType")]
    pub choice_type: Vec<i32>,
    #[serde(rename = "eventDescNameTextId")]
    pub event_desc_name_text_id: i32,
    #[serde(rename = "eventNameTextId")]
    pub event_name_text_id: i32,
    #[serde(rename = "eventSuccessRate")]
    pub event_success_rate: Vec<i32>,
    #[serde(rename = "exitChoiceSwitch")]
    pub exit_choice_switch: Option<i32>,
    #[serde(rename = "exitDescNameTextId")]
    pub exit_desc_name_text_id: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
}

pub struct RleventtableTable {
    records: Vec<Rleventtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl RleventtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rleventtable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Rleventtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Rleventtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rleventtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Rleventtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
