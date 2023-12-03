use actix_web::{get, web, HttpResponse, Responder};
use ws_scraper::card::get_card_details;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_expansions);
}

#[get("/users")]
async fn get_expansions() -> impl Responder {
    get_card_details("https://ws-tcg.com/cardlist/?cardno=ID/W13-124&l").await.unwrap();

    HttpResponse::Ok().json(vec!["a", "b", "c"])
}
