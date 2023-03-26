/**
 * @Author: @iamrezamousavi
 * @Date:   2023-03-24 17:43:42
 * @Last Modified by:   @iamrezamousavi
 * @Last Modified time: 2023-03-26 21:16:49
 */
use actix_web::{
    delete, get, middleware, patch, post, web, App, HttpResponse, HttpServer,
};
use env_logger;
use serde::Deserialize;

use crate::{
    db::{DBError, Database, TodoItem},
    Message,
};

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

pub struct TodoServer {
    host: String,
    port: u16,
}

impl TodoServer {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    #[actix_web::main(arg)]
    pub async fn run(&self) -> std::io::Result<()> {
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
        .bind((self.host.clone(), self.port))?
        .run()
        .await
    }
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    match data.database.list() {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => match err {
            DBError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(msg)
            }
            // Doesn't happen
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

#[get("/{id}/")]
async fn get_by_id(
    id: web::Path<u32>,
    data: web::Data<AppState>,
) -> HttpResponse {
    match data.database.get(*id) {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => match err {
            DBError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            DBError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(msg)
            }
            // Doesn't happen
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

#[post("/")]
async fn insert(
    todo: web::Json<TodoCreateReq>,
    data: web::Data<AppState>,
) -> HttpResponse {
    match data.database.insert(TodoItem::new(&todo.title)) {
        Ok(todo) => HttpResponse::Created().json(todo),
        Err(err) => match err {
            DBError::Conflict(msg) => HttpResponse::Conflict().json(msg),
            DBError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(msg)
            }
            // Doesn't happen
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
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
    match data.database.update(&todo) {
        Ok(_) => HttpResponse::Ok().json(todo),
        Err(err) => match err {
            DBError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            DBError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(msg)
            }
            DBError::Conflict(msg) => HttpResponse::Conflict().json(msg),
        },
    }
}

#[delete("/{id}/")]
async fn delete(id: web::Path<u32>, data: web::Data<AppState>) -> HttpResponse {
    match data.database.delete(*id) {
        Ok(_) => HttpResponse::Ok().json(Message {
            message: String::from("Item was deleted"),
        }),
        Err(err) => match err {
            DBError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            DBError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(msg)
            }
            // Doesn't happen
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
