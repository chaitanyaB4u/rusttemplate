use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};

use crate::whatsapp_models::{
    message::{Message, Text},
    payload_template::{self, Root},
    token::TokenRequest,
    whatsapp_client::WhatasppClient,
    whatsapp_error::WhatsappError,
};
use std::env;

pub fn verification_token(info: web::Query<TokenRequest>) -> Result<HttpResponse, Error> {
    println!("enter the verifiction token");
    let (token, challenge) = (info.0.token, info.0.challenge);
    setup();
    let access_token = std::env::var("MY_TOKEN").expect("The access token is not present");

    if token.eq(&access_token) {
        return Ok(HttpResponse::Ok().body(challenge.to_string()));
    }

    Ok(HttpResponse::Forbidden().body("Invalid Application token"))
}

pub async fn text_load(_request: HttpRequest, pay_load: String) -> Result<HttpResponse, Error> {
    println!("enter post end point");
    let object: Root = serde_json::from_str(&pay_load)?;
    let from = object.entry[0].changes[0].value.messages[0].from.clone();
    let message: payload_template::Message = object.entry[0].changes[0].value.messages[0].clone();
    let message_id = message.id.clone();

    if let Some(value) = message.image {
        let input_image = value;
        setup();
        let env_img_url =
            std::env::var("WHATSAPP_IMAGE_API_URL").expect("whatsapp get image api url not found");
        let url = format!("{}{}", env_img_url, input_image.id);
        let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
            .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
        let client = WhatasppClient::new(&access_token, &message_id);
        let response = client.get_image_url(&url).await;

        if let Ok(value) = response {
            println!("{:?}", value);
            let replay = "received image ";
            send_text_message_works(from, replay.to_string(), &message_id).await;
        }
    } else if let Some(value) = message.text {
        let input_text = value;
        dbg!(&input_text);
        let replay = "Welcome to numed - labs";
        let response = send_text_message_works(from, replay.to_string(), &message_id).await;
    }

    Ok(HttpResponse::Ok().body(pay_load))
}

async fn send_text_message_works(
    from: String,
    inp_msg: String,
    message_id: &str,
) -> Result<(), WhatsappError> {
    setup();
    let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
        .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
    let to = from;
    // std::env::var("WHATSAPP_SEND_TO").expect("Missing environment variable WHATSAPP_SEND_TO");
    let text = Text::new(&inp_msg);
    let message = Message::from_text(&to, text);
    let client = WhatasppClient::new(&access_token, message_id);
    let response = client.send_message(&message).await?;

    // assert_eq!(response.messages.len(), 1);
    Ok(())
}

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}
