use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("v1").service(health)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
