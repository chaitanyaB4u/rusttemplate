use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};

use crate::whatsapp_models::{
    message::{Message, Text},
    payload_template::{self, Root},
    token::TokenRequest,
    whatsapp_client::WhatsAppClient,
    whatsapp_error::WhatsappError,
};
use std::env;

pub fn verification_token(info: web::Query<TokenRequest>) -> Result<HttpResponse, Error> {
    println!("enter the verification token");
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
    let type_field = object.entry[0].changes[0].value.messages[0]
        .type_field
        .as_str();
    let from = object.entry[0].changes[0].value.messages[0].from.as_str();
    let message: payload_template::Message = object.entry[0].changes[0].value.messages[0].clone();
    let message_id = message.id.as_str();

    match type_field {
        "image" => {
            let input_image = message.image.unwrap();
            setup();
            let env_img_url = std::env::var("WHATSAPP_IMAGE_API_URL")
                .expect("whatsapp get image api url not found");
            let url = format!("{}{}", env_img_url, input_image.id);
            let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
                .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
            let client = WhatsAppClient::new(&access_token, message_id);
            let response = client.get_image_url(&url).await;

            if let Ok(value) = response {
                println!("{:?}", value);
                let replay = "received image ";
                send_text_message_works(from, replay, message_id).await;
            }
        }
        "text" => {
            let input_text = message.text.unwrap();
            let replay = "Welcome to chaitanya tech";
            let response = send_text_message_works(from, replay, message_id).await;
        }
        _ => {
            println!("Unsupported formate");
        }
    }

    Ok(HttpResponse::Ok().body(pay_load))
}

async fn send_text_message_works(
    from: &str,
    input_msg: &str,
    message_id: &str,
) -> Result<(), WhatsappError> {
    setup();
    let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
        .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
    let to = from;
    let text = Text::new(input_msg);
    let message = Message::from_text(to, text);
    let client = WhatsAppClient::new(&access_token, message_id);
    let _response = client.send_message(&message).await?;

    Ok(())
}

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}
