use serde::{Deserialize, Serialize};
/**
 * @Author: @iamrezamousavi
 * @Date:   2023-03-24 05:26:55
 * @Last Modified by:   @iamrezamousavi
 * @Last Modified time: 2023-03-24 19:45:17
 */
use sqlite;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    id: Option<i64>,
    title: String,
    is_done: bool,
}

impl TodoItem {
    pub fn new(title: &str) -> Self {
        Self {
            id: None,
            title: title.to_string(),
            is_done: false,
        }
    }
}

pub struct Database {
    filename: String,
}

impl Database {
    pub fn new(filename: String) -> Self {
        let ret = Self { filename };
        ret.init();
        ret
    }

    pub fn init(&self) {
        let connection = sqlite::open(self.filename.clone()).unwrap();

        let query = "
        CREATE TABLE IF NOT EXISTS todo (
            id     INTEGER PRIMARY KEY AUTOINCREMENT,
            title  TEXT NOT NULL,
            isdone INTEGER DEFAULT 0
        );
        ";
        connection.execute(query).unwrap();
    }
    pub fn insert(&self, item: TodoItem) -> Result<TodoItem, String> {
        let connection = sqlite::open(self.filename.clone()).unwrap();

        let query = format!(
            "INSERT INTO todo (title, isdone) VALUES ('{}', {});",
            item.title, item.is_done,
        );
        connection.execute(query).unwrap();

        self.get(item)
    }
    pub fn update(&self, item: TodoItem) {
        let connection = sqlite::open(self.filename.clone()).unwrap();

        let query = format!(
            "UPDATE todo
                SET title = {},
                    isdone = {}
                WHERE id = {}",
            item.title,
            item.is_done,
            item.id.unwrap()
        );
        connection.execute(query).unwrap();
    }
    pub fn delete(&self, item: TodoItem) {
        let connection = sqlite::open(self.filename.clone()).unwrap();

        let query = format!("DELETE FROM todo WHERE id = {}", item.id.unwrap());
        connection.execute(query).unwrap();
    }
    pub fn get(&self, item: TodoItem) -> Result<TodoItem, String> {
        let connection = sqlite::open(self.filename.clone()).unwrap();

        let query = format!(
            "
            SELECT * FROM todo
            WHERE title = '{}'
            AND isdone = {};",
            item.title, item.is_done as i8
        );

        for row in connection.prepare(query).unwrap().into_iter() {
            let row = row.unwrap();
            return Ok(TodoItem {
                id: Some(row.read::<i64, _>("id")),
                title: row.read::<&str, _>("title").to_string(),
                is_done: row.read::<i64, _>("isdone") != 0,
            });
        }
        return Err("Not Found".to_string());
    }
    pub fn list(&self) -> Vec<TodoItem> {
        let connection = sqlite::open(self.filename.clone()).unwrap();

        let mut list = vec![];
        let query = "SELECT * FROM todo;";
        for row in connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap())
        {
            list.push(TodoItem {
                id: Some(row.read::<i64, _>("id")),
                title: row.read::<&str, _>("title").to_string(),
                is_done: row.read::<i64, _>("isdone") != 0,
            })
        }
        list
    }
}
