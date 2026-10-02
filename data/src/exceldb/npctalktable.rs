// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npctalktable {
    #[serde(rename = "bubbleType", default)]
    pub bubble_type: Option<i32>,
    #[serde(rename = "faceIllustName", default)]
    pub face_illust_name: Option<String>,
    #[serde(rename = "groupId", default)]
    pub group_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "motion", default)]
    pub motion: Option<String>,
    #[serde(rename = "npcDialogStoryTextId", default)]
    pub npc_dialog_story_text_id: i32,
    #[serde(rename = "npcNameTextId", default)]
    pub npc_name_text_id: i32,
    #[serde(rename = "packId", default)]
    pub pack_id: Option<i32>,
    #[serde(rename = "speakerName", default)]
    pub speaker_name: Option<i32>,
    #[serde(rename = "tabType", default)]
    pub tab_type: Option<i32>,
    #[serde(rename = "voiceResourceName", default)]
    pub voice_resource_name: Option<String>,
}

pub struct NpctalktableTable {
    records: Vec<Npctalktable>,
    by_id: HashMap<i32, usize>,
    by_pack: HashMap<(i32, i32), usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl NpctalktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Npctalktable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_pack = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_pack.insert((record.pack_id.unwrap_or(1), record.id), idx);
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
    pub fn get(&self, id: i32) -> Option<&Npctalktable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    /// Pack-scoped lookup — use this over `get()` for any new call site,
    /// since `id` collides across packs (see `pack_id` field doc).
    #[inline]
    pub fn get_by_pack(&self, pack_id: i32, id: i32) -> Option<&Npctalktable> {
        self.by_pack.get(&(pack_id, id)).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Npctalktable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Npctalktable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Npctalktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
