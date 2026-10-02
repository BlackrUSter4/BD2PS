// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldrewardobjectgrouptable {
    #[serde(rename = "buffId", default)]
    pub buff_id: Option<i32>,
    #[serde(rename = "castingSec", default)]
    pub casting_sec: Option<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "miniMapViewFlag", default)]
    pub mini_map_view_flag: Option<i32>,
    #[serde(rename = "resetType", default)]
    pub reset_type: Option<i32>,
    #[serde(rename = "rewardGroupId", default)]
    pub reward_group_id: Option<i32>,
    #[serde(rename = "rewardIcon", default)]
    pub reward_icon: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "uiLocalTextId", default)]
    pub ui_local_text_id: i32,
    /// Not a real client-sent field; synthesized at import time from which
    /// server/Data/PACK/<n>/ folder a row came from, since `id` is only
    /// unique within a single pack, not globally. See CLIENT_UPDATE.md's
    /// 2026-10-02 "(packId, id) composite key" entries.
    #[serde(rename = "PackId", default)]
    pub pack_id: i32,
}

pub struct FieldrewardobjectgrouptableTable {
    records: Vec<Fieldrewardobjectgrouptable>,
    by_id: HashMap<i32, usize>,
    by_pack: HashMap<(i32, i32), usize>,
}

impl FieldrewardobjectgrouptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldrewardobjectgrouptable> = serde_json::from_str(&json)?;

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
    pub fn get(&self, id: i32) -> Option<&Fieldrewardobjectgrouptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    /// Pack-scoped lookup — use this over `get()` for any new call site,
    /// since `id` collides across packs (see `pack_id` field doc).
    #[inline]
    pub fn get_by_pack(&self, pack_id: i32, id: i32) -> Option<&Fieldrewardobjectgrouptable> {
        self.by_pack.get(&(pack_id, id)).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldrewardobjectgrouptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldrewardobjectgrouptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
