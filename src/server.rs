/**
 * @Author: @iamrezamousavi
 * @Date:   2023-03-24 17:43:42
 * @Last Modified by:   @iamrezamousavi
 * @Last Modified time: 2023-03-25 20:58:01
 */
use actix_web::{delete, get, middleware, patch, post, web, App, HttpResponse, HttpServer};
use env_logger;
use serde::Deserialize;

use crate::db::{Database, TodoItem};

struct AppState {
    database: Database,
}

#[derive(Debug, Deserialize)]
struct TodoCreateReq {
    title: String,
}

#[derive(Debug, Deserialize)]
struct TodoUpdateReq {
    title: String,
    is_done: u8,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let list = &data.database.list();
    HttpResponse::Ok().json(list)
}

#[get("/{id}/")]
async fn get_by_id(id: web::Path<u32>, data: web::Data<AppState>) -> HttpResponse {
    let todo = data.database.get(*id).unwrap();
    HttpResponse::Ok().json(todo)
}

#[post("/")]
async fn insert(todo: web::Json<TodoCreateReq>, data: web::Data<AppState>) -> HttpResponse {
    let todo = data.database.insert(TodoItem::new(&todo.title)).unwrap();
    HttpResponse::Ok().json(todo)
}

#[patch("/{id}/")]
async fn update(
    id: web::Path<u32>,
    todo: web::Json<TodoUpdateReq>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let todo = TodoItem {
        id: Some(*id),
        title: todo.title.clone(),
        is_done: todo.is_done,
    };
    data.database.update(&todo);
    HttpResponse::Ok().json(todo)
}

#[delete("/{id}/")]
async fn delete(
    id: web::Path<u32>,
    todo: web::Json<TodoUpdateReq>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let todo = TodoItem {
        id: Some(*id),
        title: todo.title.clone(),
        is_done: todo.is_done,
    };
    data.database.delete(&todo);
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
            .service(get_by_id)
            .service(insert)
            .service(update)
            .service(delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
