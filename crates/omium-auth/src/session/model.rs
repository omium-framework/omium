//! In-memory session payload (Laravel `Session` bag analogue).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::SessionId;

/// Session data loaded from a store and attached to an HTTP request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    id: SessionId,
    user_id: Option<i64>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    #[serde(default)]
    data: HashMap<String, Value>,
    last_activity: i64,
    #[serde(skip)]
    dirty: bool,
    #[serde(skip)]
    destroyed: bool,
}

impl Session {
    /// Starts a new empty session with a fresh id.
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: SessionId::generate(),
            user_id: None,
            ip_address: None,
            user_agent: None,
            data: HashMap::new(),
            last_activity: now_unix(),
            dirty: true,
            destroyed: false,
        }
    }

    #[must_use]
    pub fn id(&self) -> &SessionId {
        &self.id
    }

    #[must_use]
    pub fn user_id(&self) -> Option<i64> {
        self.user_id
    }

    pub fn set_user_id(&mut self, user_id: Option<i64>) {
        self.user_id = user_id;
        self.mark_dirty();
    }

    #[must_use]
    pub fn is_destroyed(&self) -> bool {
        self.destroyed
    }

    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.dirty || self.destroyed
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
        self.last_activity = now_unix();
    }

    /// Assigns a new session id (call after login — fixation mitigation).
    pub fn regenerate_id(&mut self) {
        self.id = SessionId::generate();
        self.mark_dirty();
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
        self.data.clear();
        self.user_id = None;
        self.mark_dirty();
    }

    pub fn set_client_hints(&mut self, ip_address: Option<String>, user_agent: Option<String>) {
        self.ip_address = ip_address;
        self.user_agent = user_agent;
        self.mark_dirty();
    }

    #[must_use]
    pub fn ip_address(&self) -> Option<&str> {
        self.ip_address.as_deref()
    }

    #[must_use]
    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }

    #[must_use]
    pub fn last_activity(&self) -> i64 {
        self.last_activity
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn put<T: Serialize>(&mut self, key: &str, value: T) {
        if let Ok(json) = serde_json::to_value(value) {
            self.data.insert(key.to_string(), json);
            self.mark_dirty();
        }
    }

    pub fn forget(&mut self, key: &str) {
        if self.data.remove(key).is_some() {
            self.mark_dirty();
        }
    }

    pub(crate) fn data_map(&self) -> &HashMap<String, Value> {
        &self.data
    }

    pub(crate) fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    pub(crate) fn from_persisted(
        id: SessionId,
        user_id: Option<i64>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        data: HashMap<String, Value>,
        last_activity: i64,
    ) -> Self {
        Self {
            id,
            user_id,
            ip_address,
            user_agent,
            data,
            last_activity,
            dirty: false,
            destroyed: false,
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[must_use]
pub fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}
