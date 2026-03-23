use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// add TODO
// inputs: content, deadline
// output: "Added" or "Error: the TODO isn't added"
// insert into database {ID, content, created_at, deadline, done}
// ID, created_at, done is automatically inserted.
#[derive(Deserialize)]
pub struct NewToDo {
    content: String,
    deadline: String,
}
pub async fn add(State(state): State<AppState>, Json(payload): Json<NewToDo>) -> &'static str {
    let result = sqlx::query!(
        "INSERT INTO todo_list (content, deadline) VALUES (?, ?)",
        payload.content,
        payload.deadline
    )
    .execute(&state.database_pool)
    .await;

    match result {
        Ok(_) => "Added!",
        Err(_) => "Error: the TODO isn't added.",
    }
}

// delete TODO
// inputs: ID
// output: "Delete the TODO" or "Error: the TODO isn't deleted"
pub async fn delete(State(state): State<AppState>, Path(id): Path<i32>) -> &'static str {
    let result = sqlx::query!("DELETE FROM todo_list WHERE id = ?", id)
        .execute(&state.database_pool)
        .await;

    match result {
        Ok(_) => "Delete the ToDo.",
        Err(_) => "Error: the ToDo isn't deleted.",
    }
}

// check TODO
//inputs: None
//output: remaining TODOs as Json
#[derive(Serialize)]
struct ToDo {
    id: i32,
    content: String,
    created_at: Option<NaiveDate>,
    deadline: Option<NaiveDate>,
    done: Option<i8>,
}
pub async fn check(State(state): State<AppState>) -> Json<Vec<ToDo>> {
    let result = sqlx::query_as!(ToDo, "SELECT * FROM todo_list")
        .fetch_all(&state.database_pool)
        .await;

    match result {
        Ok(todos) => Json(todos),
        Err(_) => Json(vec![ToDo {
            id: -1,
            content: String::from("Error"),
            created_at: None,
            deadline: None,
            done: None,
        }]),
    }
}
