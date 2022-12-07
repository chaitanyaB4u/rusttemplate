use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadMessage {
    #[serde(rename = "messaging_product")]
    pub messaging_product: String,
    pub status: String,
    #[serde(rename = "message_id")]
    pub message_id: String,
}
impl ReadMessage {
    pub fn new(message_id: &str) -> Self {
        Self {
            messaging_product: "whatsapp".to_string(),
            status: "read".to_string(),
            message_id: message_id.to_string(),
        }
    }
}
