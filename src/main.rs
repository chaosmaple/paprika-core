pub mod test;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[allow(warnings, unused)]
pub mod db;
use db::PrismaClient;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(web::scope("/test").configure(test::config))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
