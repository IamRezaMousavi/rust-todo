/**
 * @Author: @iamrezamousavi
 * @Date:   2023-03-24 05:26:55
 * @Last Modified by:   @iamrezamousavi
 * @Last Modified time: 2023-03-25 21:20:44
 */
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

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

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(filename: String) -> Self {
        let ret = Self {
            conn: Connection::open(filename).unwrap(),
        };
        ret.init();
        ret
    }

    pub fn init(&self) {
        let query = "
        CREATE TABLE IF NOT EXISTS todo (
            id     INTEGER PRIMARY KEY AUTOINCREMENT,
            title  TEXT NOT NULL,
            isdone INTEGER DEFAULT 0
        );
        ";
        self.conn.execute(query, ()).unwrap();
    }
    pub fn insert(&self, item: TodoItem) -> Result<TodoItem, String> {
        self.conn
            .execute(
                "INSERT INTO todo (title, isdone) VALUES (?1, ?2);",
                (&item.title, &item.is_done.to_string()),
            )
            .unwrap();

        let last_id = self.conn.last_insert_rowid() as u32;

        Ok(TodoItem {
            id: Some(last_id),
            title: item.title,
            is_done: item.is_done,
        })
    }
    pub fn update(&self, item: &TodoItem) {
        self.conn
            .execute(
                "UPDATE todo
                SET title = ?1,
                    isdone = ?2
                WHERE id = ?3;",
                (
                    &item.title,
                    &item.is_done.to_string(),
                    &item.id.unwrap().to_string(),
                ),
            )
            .unwrap();
    }
    pub fn delete(&self, item: &TodoItem) {
        self.conn
            .execute(
                "DELETE FROM todo WHERE id = ?1",
                (&item.id.unwrap().to_string(),),
            )
            .unwrap();
    }

    pub fn list(&self) -> Vec<TodoItem> {
        let mut todos = vec![];
        for todo in self
            .conn
            .prepare("SELECT id, title, isdone FROM todo;")
            .unwrap()
            .query_map([], |row| {
                Ok(TodoItem {
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    is_done: row.get(2).unwrap(),
                })
            })
            .unwrap()
        {
            todos.push(todo.unwrap());
        }
        todos
    }

    pub fn get(&self, id: u32) -> Result<TodoItem, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, isdone FROM todo WHERE id = ?1")
            .unwrap();
        let todo_iter = stmt
            .query_map([&id], |row| {
                Ok(TodoItem {
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    is_done: row.get(2).unwrap(),
                })
            })
            .unwrap();
        for todo in todo_iter {
            return Ok(todo.unwrap());
        }
        Err(String::from("Not match"))
    }
}
