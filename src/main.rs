mod model;
mod utils;

use tokio::net::TcpListener;
use entity::todo;

use crate::{
    model::{TodoModel, CreateTodoModel, UpdateTodoModel},
    utils::api_error::APIError,
};

use axum::{
    Json, Router,
    extract::{ State, Path},
    http::StatusCode,
    routing::{get, post, put, delete},
};

use sea_orm::{
    ActiveModelTrait, Database, DatabaseConnection, EntityTrait, ActiveValue::Set
};

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let conn = Database::connect(&db_url).await.unwrap();

    let state = AppState { conn };

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/todos", get(list_todos))
        .route("/create_todo", post(create_todo))
        .route("/:id/update_todo", put(update_todo))
        .route("/:id/delete_todo", delete(delete_todo))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn list_todos( State(state): State<AppState>) -> Result<Json<Vec<TodoModel>>, APIError> {
    let todos = todo::Entity::find().all(&state.conn).await
        .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
        .into_iter()
        .map(|item| TodoModel{
            id: item.id,
            todo: item.todo,
            completed: item.completed,
        })
        .collect();

    Ok(Json(todos))
}

async fn create_todo( State(state): State<AppState>, Json(data): Json<CreateTodoModel> ) -> Result<(), APIError> {
    let todo_model = todo::ActiveModel{
        todo: Set(data.todo),
        completed: Set(data.completed),
            ..Default::default()
    };

    todo_model.insert(&state.conn).await
        .map_err(|err| APIError {message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;

    Ok(())
}

async fn update_todo( State(state): State<AppState>, Path(id): Path<i32>, Json(data): Json<UpdateTodoModel> ) -> Result<(), APIError> {
    let todo: todo::ActiveModel = todo::Entity::find_by_id(id).one(&state.conn).await
        .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
        .ok_or(APIError { message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND, error_code: Some(44) })?
        .into();

    todo::ActiveModel{
        id: todo.id,
        todo: Set(data.todo),
        completed: Set(data.completed),
    }.update(&state.conn).await
        .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;

    Ok(())
}

async fn delete_todo( State(state): State<AppState>, Path(id): Path<i32>) -> Result<(), APIError> {
    let todo = todo::Entity::find_by_id(id).one(&state.conn).await
        .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
        .ok_or(APIError { message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND, error_code: Some(44) })?;

    todo::Entity::delete_by_id(todo.id).exec(&state.conn)
        .await
        .map_err(
            |err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)}
        )?;

    Ok(())
}
