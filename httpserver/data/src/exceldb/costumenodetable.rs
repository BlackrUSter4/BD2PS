// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Costumenodetable {
    #[serde(rename = "activeItemCount")]
    pub active_item_count: Vec<i32>,
    #[serde(rename = "activeItemId")]
    pub active_item_id: Vec<i32>,
    #[serde(rename = "activeItemType")]
    pub active_item_type: Vec<i32>,
    #[serde(rename = "addBuffId")]
    pub add_buff_id: Option<i32>,
    #[serde(rename = "addBuffModifyBuffMagicValue")]
    pub add_buff_modify_buff_magic_value: Option<f32>,
    #[serde(rename = "addBuffModifyBuffTurn")]
    pub add_buff_modify_buff_turn: Option<i32>,
    #[serde(rename = "addBuffModifyBuffValue")]
    pub add_buff_modify_buff_value: Option<f32>,
    #[serde(rename = "addBuffOrder")]
    pub add_buff_order: Option<i32>,
    #[serde(rename = "addBuffTextOrder")]
    pub add_buff_text_order: Option<i32>,
    #[serde(rename = "attackRange")]
    pub attack_range: Option<i32>,
    #[serde(rename = "attackRangeCount")]
    pub attack_range_count: Option<i32>,
    #[serde(rename = "conditionAddBuffModifyBuffOrder")]
    pub condition_add_buff_modify_buff_order: Option<i32>,
    #[serde(rename = "conditionAddBuffModifyBuffTurn")]
    pub condition_add_buff_modify_buff_turn: Option<i32>,
    #[serde(rename = "conditionAddBuffModifyBuffValue")]
    pub condition_add_buff_modify_buff_value: Option<f32>,
    #[serde(rename = "conditionGrade")]
    pub condition_grade: Option<i32>,
    #[serde(rename = "conditionNodeId")]
    pub condition_node_id: Option<Vec<i32>>,
    #[serde(rename = "cooldownDecreaseValue")]
    pub cooldown_decrease_value: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "modifyBuffMagicValue")]
    pub modify_buff_magic_value: Option<f32>,
    #[serde(rename = "modifyBuffOrder")]
    pub modify_buff_order: Option<i32>,
    #[serde(rename = "modifyBuffTurn")]
    pub modify_buff_turn: Option<i32>,
    #[serde(rename = "modifyBuffValue")]
    pub modify_buff_value: Option<f32>,
    #[serde(rename = "nodeGroupType")]
    pub node_group_type: i32,
    #[serde(rename = "nodeType")]
    pub node_type: i32,
    #[serde(rename = "spDecreaseValue")]
    pub sp_decrease_value: Option<i32>,
    #[serde(rename = "statType")]
    pub stat_type: Option<i32>,
    #[serde(rename = "statValue")]
    pub stat_value: Option<f32>,
}

pub struct CostumenodetableTable {
    records: Vec<Costumenodetable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl CostumenodetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Costumenodetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Costumenodetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Costumenodetable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Costumenodetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Costumenodetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
