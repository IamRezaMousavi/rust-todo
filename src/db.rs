/**
 * @Author: @iamrezamousavi
 * @Date:   2023-03-24 05:26:55
 * @Last Modified by:   @iamrezamousavi
 * @Last Modified time: 2023-03-26 19:50:04
 */
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

use super::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: Option<u32>,
    pub title: String,
    pub is_done: u8,
}

impl TodoItem {
    pub fn new(title: &str) -> Self {
        Self {
            id: None,
            title: title.to_string(),
            is_done: 0,
        }
    }
}

#[derive(Debug)]
pub enum DBError {
    NotFound(Message),
    Conflict(Message),
    InternalError(Message),
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(filename: String) -> Self {
        let ret = Self {
            conn: match Connection::open(filename) {
                Ok(conn) => conn,
                // panic if open in-memory failed too
                Err(_) => Connection::open_in_memory().unwrap(),
            },
        };
        // panic if couldn't create table
        ret.init().unwrap();
        ret
    }

    pub fn init(&self) -> Result<(), DBError> {
        let query = "
        CREATE TABLE IF NOT EXISTS todo (
            id     INTEGER PRIMARY KEY AUTOINCREMENT,
            title  TEXT UNIQUE NOT NULL,
            isdone INTEGER DEFAULT 0
        );
        ";
        match self.conn.execute(query, ()) {
            Ok(_) => Ok(()),
            Err(err) => Err(DBError::InternalError(Message {
                message: format!(
                    "Failed to execute create table SQL statement: {err}"
                ),
            })),
        }
    }
    pub fn insert(&self, item: TodoItem) -> Result<TodoItem, DBError> {
        let mut stmt = match self
            .conn
            .prepare("INSERT INTO todo (title, isdone) VALUES (?1, ?2);")
        {
            Ok(stmt) => stmt,
            Err(err) => {
                return Err(DBError::InternalError(Message {
                    message: format!(
                        "Failed to prepare insert SQL statement: {err}"
                    ),
                }));
            }
        };
        let res = stmt.execute([&item.title, &item.is_done.to_string()]);
        match res {
            Ok(_) => {
                let last_id = self.conn.last_insert_rowid() as u32;
                Ok(TodoItem {
                    id: Some(last_id),
                    title: item.title,
                    is_done: item.is_done,
                })
            }
            Err(err) => match err {
                rusqlite::Error::SqliteFailure { 0: sqlerr, 1: _ } => {
                    match sqlerr.code {
                        rusqlite::ErrorCode::ConstraintViolation => {
                            Err(DBError::Conflict(Message {
                                message: String::from("Item already exists"),
                            }))
                        }
                        _ => Err(DBError::InternalError(Message {
                            message: format!(
                                "Failed to execute insert SQL statement: {err}"
                            ),
                        })),
                    }
                }
                _ => Err(DBError::InternalError(Message {
                    message: format!(
                        "Failed to execute insert SQL statement: {err}"
                    ),
                })),
            },
        }
    }

    pub fn update(&self, item: &TodoItem) -> Result<(), DBError> {
        match self.conn.execute(
            "UPDATE todo
                SET title = ?1,
                    isdone = ?2
                WHERE id = ?3;",
            (
                &item.title,
                &item.is_done.to_string(),
                &item.id.unwrap().to_string(),
            ),
        ) {
            Ok(rows) => match rows {
                1 => Ok(()),
                0 => Err(DBError::NotFound(Message {
                    message: String::from("Item not found"),
                })),
                _ => Err(DBError::InternalError(Message {
                    message: format!("DBError, Rows Changed: {rows}"),
                })),
            },
            Err(err) => match err {
                rusqlite::Error::SqliteFailure { 0: sqlerr, 1: _ } => {
                    match sqlerr.code {
                        rusqlite::ErrorCode::ConstraintViolation => {
                            Err(DBError::Conflict(Message {
                                message: String::from("Duplicate title"),
                            }))
                        }
                        _ => Err(DBError::InternalError(Message {
                            message: format!("DBError: {err}"),
                        })),
                    }
                }
                _ => Err(DBError::InternalError(Message {
                    message: format!("DBError: {err}"),
                })),
            },
        }
    }

    pub fn delete(&self, id: u32) -> Result<(), DBError> {
        match self
            .conn
            .execute("DELETE FROM todo WHERE id = ?1", (id.to_string(),))
        {
            Ok(rows) => match rows {
                1 => Ok(()),
                0 => Err(DBError::NotFound(Message {
                    message: String::from("Item not found"),
                })),
                _ => Err(DBError::InternalError(Message {
                    message: format!("DBError, Rows Changed: {rows}"),
                })),
            },
            Err(err) => Err(DBError::InternalError(Message {
                message: format!("DBError: {err}"),
            })),
        }
    }

    pub fn list(&self) -> Result<Vec<TodoItem>, DBError> {
        let mut output = vec![];

        let mut stmt =
            match self.conn.prepare("SELECT id, title, isdone FROM todo;") {
                Ok(stmt) => stmt,
                Err(err) => {
                    return Err(DBError::InternalError(Message {
                        message: format!(
                            "Failed to prepare list SQL statement: {err}"
                        ),
                    }))
                }
            };
        let x = stmt.query_map([], |row| {
            Ok(TodoItem {
                id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                is_done: row.get(2).unwrap(),
            })
        });
        match x {
            Ok(todos) => {
                for todo in todos {
                    output.push(todo.unwrap());
                }
                Ok(output)
            }
            Err(err) => Err(DBError::InternalError(Message {
                message: format!("DBError: {err}"),
            })),
        }
    }

    pub fn get(&self, id: u32) -> Result<TodoItem, DBError> {
        let mut stmt = match self
            .conn
            .prepare("SELECT id, title, isdone FROM todo WHERE id = ?1")
        {
            Ok(stmt) => stmt,
            Err(err) => {
                return Err(DBError::InternalError(Message {
                    message: format!(
                        "Failed to prepare get SQL statement: {err}"
                    ),
                }))
            }
        };
        let x = stmt.query_map([&id], |row| {
            Ok(TodoItem {
                id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                is_done: row.get(2).unwrap(),
            })
        });
        match x {
            Ok(todos) => match todos.into_iter().next() {
                Some(t) => match t {
                    Ok(todo) => Ok(todo),
                    Err(err) => Err(DBError::InternalError(Message {
                        message: format!("Error : {err}"),
                    })),
                },
                None => Err(DBError::NotFound(Message {
                    message: String::from("Item not found"),
                })),
            },
            Err(err) => Err(DBError::InternalError(Message {
                message: format!("DBError: {err}"),
            })),
        }
    }
}
