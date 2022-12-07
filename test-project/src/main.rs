use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error};
use whatsapp::{verification_token, text_load};
use whatsapp_models::token::TokenRequest;
pub mod whatsapp;
pub mod whatsapp_models;
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn verify_token(info: web::Query<TokenRequest>) -> Result<HttpResponse, Error> {
    verification_token(info)
}

async fn post_message(_request: HttpRequest, payload: String) -> Result<HttpResponse, Error> {
    text_load(_request, payload).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("webhook", web::get().to(verify_token))
            .route("webhook", web::post().to(post_message))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}