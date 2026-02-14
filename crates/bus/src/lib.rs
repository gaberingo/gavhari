use serde_json::Value;
use shared::EventMeta;

pub trait EventBus {
    fn publish(&self, meta: &EventMeta, payload: &Value);
}
