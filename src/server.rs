/**
 * @Author: @iamrezamousavi
 * @Date:   2023-03-24 17:43:42
 * @Last Modified by:   @iamrezamousavi
 * @Last Modified time: 2023-03-24 20:00:13
 */
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use env_logger;
use serde::Deserialize;

use crate::db::{Database, TodoItem};

struct AppState {
    database: Database,
}

#[derive(Debug, Deserialize)]
struct TodoReq {
    title: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let list = &data.database.list();
    HttpResponse::Ok().json(list)
}

#[post("/")]
async fn insert(todo: web::Json<TodoReq>, data: web::Data<AppState>) -> HttpResponse {
    let todo = data.database.insert(TodoItem::new(&todo.title)).unwrap();
    HttpResponse::Ok().json(todo)
}

#[actix_web::main]
pub async fn servermain() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                database: Database::new(String::from("data.db")),
            }))
            .wrap(middleware::Logger::default())
            .service(index)
            .service(insert)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
