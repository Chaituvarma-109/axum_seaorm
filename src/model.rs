use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct TodoModel {
    pub id: i32,
    pub todo: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTodoModel {
    pub todo: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateTodoModel {
    pub todo: String,
    pub completed: bool,
}
