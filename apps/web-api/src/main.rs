use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health))
        .bind("127.0.0.1:8081")?
        .run()
        .await
}

