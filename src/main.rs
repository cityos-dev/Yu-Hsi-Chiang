mod upload;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body(include_str!("index.html"))
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(home).service(
            web::scope("v1")
                .service(health)
                .service(upload::upload_file),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
