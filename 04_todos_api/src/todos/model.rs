use crate::db;
use crate::schema::todos;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use dropshot::HttpError;

#[derive(Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq, JsonSchema)]
#[table_name = "todos"]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub checked: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, AsChangeset, Debug, JsonSchema)]
#[table_name = "todos"]
pub struct TodoRequest {
    pub title: String,
    pub checked: bool
}

impl Todo {
    pub fn all() -> Result<Vec<Self>, HttpError> {
        let conn = db::connection()?;
        let b: Result<Vec<Self>, diesel::result::Error> =
            diesel::sql_query("SELECT * FROM todos ORDER BY created_at").get_results(&conn);

        Ok(b.unwrap())
    }

    pub fn create(todo: TodoRequest) -> Result<Uuid, HttpError> {
        let conn = db::connection()?;

        let todo = Todo::from(todo);
        match diesel::insert_into(todos::table)
            .values(&todo)
            .get_result::<Todo>(&conn)
        {
            Ok(v) => Ok(v.id),
            Err(_e) => Err(HttpError::for_bad_request(
                None,
                String::from("something went wrong"),
            )),
        }
    }

    pub fn update(uid: Uuid, todo: TodoRequest) -> Result<Self, HttpError> {
        let conn = db::connection()?;

        match diesel::update(todos::table)
            .filter(todos::id.eq(uid))
            .set(todo)
            .get_result(&conn)
        {
            Ok(v) => Ok(v),
            Err(_e) => Err(HttpError::for_bad_request(
                None,
                String::from("something went wrong"),
            )),
        }
    }

    pub fn delete(uid: Uuid) -> Result<usize, HttpError> {
        let conn = db::connection()?;

        match diesel::delete(todos::table.filter(todos::id.eq(uid))).execute(&conn) {
            Ok(v) => Ok(v),
            Err(_e) => Err(HttpError::for_bad_request(
                None,
                String::from("something went wrong"),
            )),
        }
    }
}

impl From<TodoRequest> for Todo {
    fn from(todo: TodoRequest) -> Self {
        Todo {
            id: Uuid::new_v4(),
            title: todo.title.trim().to_string(),
            checked: todo.checked,
            created_at: Utc::now().naive_utc()
        }
    }
}
