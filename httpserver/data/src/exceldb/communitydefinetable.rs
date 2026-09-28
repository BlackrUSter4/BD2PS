// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Communitydefinetable {
    #[serde(rename = "friendSendExpireDay")]
    pub friend_send_expire_day: i32,
    #[serde(rename = "maxFriendCount")]
    pub max_friend_count: i32,
    #[serde(rename = "maxFriendRecommendCount")]
    pub max_friend_recommend_count: i32,
    #[serde(rename = "maxFriendRecvCount")]
    pub max_friend_recv_count: i32,
    #[serde(rename = "maxFriendSendCount")]
    pub max_friend_send_count: i32,
}

pub struct CommunitydefinetableTable {
    records: Vec<Communitydefinetable>,
}

impl CommunitydefinetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Communitydefinetable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Communitydefinetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Communitydefinetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
