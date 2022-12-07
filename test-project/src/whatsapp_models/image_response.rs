use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageResponse {
    #[serde(rename = "messaging_product")]
    pub messaging_product: String,
    pub url: String,
    #[serde(rename = "mime_type")]
    pub mime_type: String,
    pub sha256: String,
    #[serde(rename = "file_size")]
    pub file_size: String,
    pub id: String,
}