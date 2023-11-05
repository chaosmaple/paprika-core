use actix_web::{get, post, web, Result};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(test_path)
        .service(test_query)
        .service(test_json);
}

#[derive(Deserialize)]
struct TestObject {
    id: u32,
    name: String,
}

#[get("/path/{id}/{name}")]
async fn test_path(path: web::Path<(u32, String)>) -> Result<String> {
    let (id, name) = path.into_inner();
    Ok(format!("id: {}, name: {}", id, name))
}

#[get("/test_query")]
async fn test_query(query: web::Query<TestObject>) -> Result<String> {
    Ok(format!("id: {}, name: {}", query.id, query.name))
}

#[post("/test_json")]
async fn test_json(query: web::Json<TestObject>) -> Result<String> {
    Ok(format!("id: {}, name: {}", query.id, query.name))
}
