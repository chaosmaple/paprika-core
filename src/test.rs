use actix_web::{get, web, HttpResponse, Responder};

use ws_db::db::{PrismaClient, ws_expansion};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_expansions);
}

#[get("/users")]
async fn get_expansions(client: web::Data<PrismaClient>) -> impl Responder {
    let users: Vec<ws_expansion::Data> = client.ws_expansion().find_many(vec![]).exec().await.unwrap();

    HttpResponse::Ok().json(users)
}
