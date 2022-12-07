use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TokenRequest {

    #[serde(rename = "hub.mode")]
    pub mode: String,

    #[serde(rename = "hub.challenge")]
    pub challenge: i32,

    #[serde(rename = "hub.verify_token")]
    pub token: String
}