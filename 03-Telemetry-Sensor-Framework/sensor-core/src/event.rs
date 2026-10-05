use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    ProcessSnapshot,
    FileObservation,
    NetworkObservation,
    SystemObservation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorEvent {
    pub event_id: String,
    pub timestamp_utc: String,
    pub host: String,
    pub os: String,
    pub source: String,
    pub event_type: EventType,
    pub process_name: Option<String>,
    pub pid: Option<u32>,
    pub parent_pid: Option<u32>,
    pub username: Option<String>,
    pub command_line: Option<String>,
    pub path: Option<String>,
    pub remote_address: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
}

impl SensorEvent {
    pub fn new(event_type: EventType, source: &str) -> Self {
        Self {
            event_id: Uuid::new_v4().to_string(),
            timestamp_utc: Utc::now().to_rfc3339(),
            host: detect_host(),
            os: std::env::consts::OS.to_string(),
            source: source.to_string(),
            event_type,
            process_name: None,
            pid: None,
            parent_pid: None,
            username: None,
            command_line: None,
            path: None,
            remote_address: None,
            notes: None,
            tags: Vec::new(),
        }
    }

    pub fn with_note(mut self, note: &str) -> Self {
        self.notes = Some(note.to_string());
        self
    }

    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
}

fn detect_host() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "unknown-host".to_string())
}
