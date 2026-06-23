use crate::websocket::{HmrMessage, HmrSender};

pub fn send_css_update(tx: &HmrSender) {
    let _ = tx.send(serde_json::to_string(&HmrMessage::css_update()).unwrap());
}
pub fn send_full_reload(tx: &HmrSender) {
    let _ = tx.send(serde_json::to_string(&HmrMessage::full_reload()).unwrap());
}
pub fn send_error(tx: &HmrSender, msg: String) {
    let _ = tx.send(serde_json::to_string(&HmrMessage::error(msg)).unwrap());
}
pub fn send_clear_error(tx: &HmrSender) {
    let _ = tx.send(serde_json::to_string(&HmrMessage::clear_error()).unwrap());
}
