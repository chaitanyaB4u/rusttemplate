use futures::future::ok;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};

use super::{
    message::Message, message_response::MessageResponse,
    whatsapp_error::WhatsappError,
};

const WHATSAPP_API_URL: &str = "https://graph.facebook.com/v15.0/105178082424775/messages";

pub struct WhatasppClient {
    access_token: String,
}

impl WhatasppClient {
    pub fn new(access_token: &str) -> Self {
        Self {
            access_token: access_token.into(),
        }
    }

    pub async fn send_message(&self, message: &Message) -> Result<MessageResponse, WhatsappError> {
        http_client::post(WHATSAPP_API_URL, &self.access_token, message)
    }
    // pub async fn get_image_url(&self,url:&str) -> Result<ImageResponse, WhatsappError> {
    //     dbg!("abc");
    //     http_client::get(url,&self.access_token)
    // }

    pub async fn get_image_url(&self, url: &str) -> Result<(), WhatsappError> {
        //http_client::get(url, &self.access_token)
        Ok(())
    }
}

mod http_client {
    use reqwest::StatusCode;
    use serde::{de::DeserializeOwned, Serialize};

    use crate::whatsapp_models::{image_response::ImageResponse, whatsapp_error::WhatsappError};

    #[tokio::main]
    pub async fn post<T, U>(url: &str, bearer_token: &str, data: &T) -> Result<U, WhatsappError>
    where
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .bearer_auth(&bearer_token)
            .json(&data)
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let json = resp.json::<U>().await?;
                Ok(json)
            }
            _ => {
                // log::warn!("{:?}", &resp);
                let error_text = &resp.text().await?;
                // log::warn!("{:?}", &error_text);
                Err(WhatsappError::UnexpectedError(error_text.to_string()))
            }
        }
    }

    #[tokio::main]
    pub async fn get(url: &str, bearer_token: &str) -> Result<ImageResponse, WhatsappError> {
        let client = reqwest::Client::builder().build()?;

        // Perform the actual execution of the network request
        let result = client.get(url).bearer_auth(&bearer_token).send().await?;

        let tests: ImageResponse = result.json().await?;

        Ok(tests)
    }
}
