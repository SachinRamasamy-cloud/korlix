use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmrMessage {
    #[serde(rename = "type")]
    pub kind: String,
    pub error: Option<String>,
}

impl HmrMessage {
    pub fn css_update() -> Self {
        Self {
            kind: "css-update".into(),
            error: None,
        }
    }
    pub fn full_reload() -> Self {
        Self {
            kind: "full-reload".into(),
            error: None,
        }
    }
    pub fn error(msg: String) -> Self {
        Self {
            kind: "error".into(),
            error: Some(msg),
        }
    }
    pub fn clear_error() -> Self {
        Self {
            kind: "clear-error".into(),
            error: None,
        }
    }
}

pub type HmrSender = broadcast::Sender<String>;
pub type HmrReceiver = broadcast::Receiver<String>;

pub fn create_hmr_channel(cap: usize) -> (HmrSender, HmrReceiver) {
    broadcast::channel(cap)
}
