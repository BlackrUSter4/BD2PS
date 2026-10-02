// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldnpcrewardtable {
    #[serde(rename = "groupId", default)]
    pub group_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "rewardId", default)]
    pub reward_id: i32,
    /// Not a real client-sent field; synthesized at import time from which
    /// server/Data/PACK/<n>/ folder a row came from, since `id` is only
    /// unique within a single pack, not globally. See CLIENT_UPDATE.md's
    /// 2026-10-02 "(packId, id) composite key" entries.
    #[serde(rename = "PackId", default)]
    pub pack_id: i32,
}

pub struct FieldnpcrewardtableTable {
    records: Vec<Fieldnpcrewardtable>,
    by_id: HashMap<i32, usize>,
    by_pack: HashMap<(i32, i32), usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl FieldnpcrewardtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldnpcrewardtable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_pack = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_pack.insert((record.pack_id, record.id), idx);
            by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_pack,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Fieldnpcrewardtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    /// Pack-scoped lookup — use this over `get()` for any new call site,
    /// since `id` collides across packs (see `pack_id` field doc).
    #[inline]
    pub fn get_by_pack(&self, pack_id: i32, id: i32) -> Option<&Fieldnpcrewardtable> {
        self.by_pack.get(&(pack_id, id)).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Fieldnpcrewardtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldnpcrewardtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldnpcrewardtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
