use crate::AppState;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ToDo {
    id: usize,
    content: String,
    date: String,
    deadline: String,
    done: bool,
}

// add TODO
// inputs: content, deadline
// output: "Added" or "Error: the TODO isn't added"
// insert into database {ID, content, date, deadline, done}
// ID, date, done is automatically inserted.
#[derive(Deserialize)]
struct NewToDo {
    content: String,
    deadline: String,
}
pub async fn add(State(state): State<AppState>, Json(payload): Json<NewToDo>) -> &'static str {}

// delete TODO
// inputs: ID
// output: "Delete the TODO" or "Error: the TODO isn't deleted"
pub async fn delete() -> &'static str {}

// check TODO
//inputs: None
//output: remaining TODOs as Json
pub async fn check() -> Json<Vec<ToDo>> {}
