// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Costumebursttable {
    #[serde(rename = "buffOrder", default)]
    pub buff_order: Vec<i32>,
    #[serde(rename = "burstBuff", default)]
    pub burst_buff: Vec<i32>,
    #[serde(rename = "burstText", default)]
    pub burst_text: Vec<i32>,
    #[serde(rename = "groupId", default)]
    pub group_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "itemCount", default)]
    pub item_count: Vec<i32>,
    #[serde(rename = "itemId", default)]
    pub item_id: Vec<i32>,
    #[serde(rename = "itemType", default)]
    pub item_type: Vec<i32>,
    #[serde(rename = "modifyAttackRange", default)]
    pub modify_attack_range: Option<i32>,
    #[serde(rename = "modifyAttackRangeCount", default)]
    pub modify_attack_range_count: Option<i32>,
    #[serde(rename = "modifyAttackRangeIndex", default)]
    pub modify_attack_range_index: Option<i32>,
    #[serde(rename = "modifyValue", default)]
    pub modify_value: Vec<f32>,
    #[serde(rename = "spReqCount", default)]
    pub sp_req_count: i32,
    #[serde(rename = "valueType", default)]
    pub value_type: Vec<i32>,
}

pub struct CostumebursttableTable {
    records: Vec<Costumebursttable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl CostumebursttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Costumebursttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Costumebursttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Costumebursttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Costumebursttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Costumebursttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
