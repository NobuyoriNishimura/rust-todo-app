use axum::Json;
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
// inputs: content, date, deadline
// output: "Added" or "Error: the TODO isn't added"
// insert into database {ID, content, date, deadline, done}
// ID: usize, content: String, date: String, deadline: String, done: bool
pub async fn add() -> &'static str {}

// delete TODO
// inputs: ID
// output: "Delete the TODO" or "Error: the TODO isn't deleted"
pub async fn delete() -> &'static str {}

// check TODO
//inputs: None
//output: remaining TODOs as Json
pub async fn check() -> Json<Vec<ToDo>> {}
