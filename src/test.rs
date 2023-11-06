use actix_web::{get, web, HttpResponse, Responder};

use crate::db::PrismaClient;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
}

#[get("/users")]
async fn get_users(client: web::Data<PrismaClient>) -> impl Responder {
    let users = client.user().find_many(vec![]).exec().await.unwrap();

    HttpResponse::Ok().json(users)
}
