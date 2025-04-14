use axum::{
    Router, 
    routing::{
        get, post,
        //  put, delete
    }, 
    Json,
    extract::Path,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, collections::HashMap};
use tokio::sync::Mutex;
use once_cell::sync::Lazy;

static DB: Lazy<Mutex<HashMap<u32, Todo>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    // .route("/", get(|| async { "Hello, World!" }));
    .route("/todos", post(create_todo).get(list_todos))
    .route("/todos/{id}", get(get_todo).put(update_todo).delete(delete_todo));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//POST /todos
async fn create_todo(Json(todo): Json<Todo>) -> Result<Json<Todo>, StatusCode> {
    let mut db = DB.lock().await;
    db.insert(todo.id, todo.clone());
    Ok(Json(todo))
}

//GET /todos
async fn list_todos() -> Result<Json<Vec<Todo>>, StatusCode> {
    let db = DB.lock().await;
    let todos: Vec<Todo> = db.values().cloned().collect();
    Ok(Json(todos))
}


//GET /todos/:id
async fn get_todo(Path(id): Path<u32>) -> Result<Json<Todo>, StatusCode> {
    let db = DB.lock().await;
    db.get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

//PUT /todos/:id
async fn update_todo(Path(id): Path<u32>, Json(payload): Json<Todo>) -> Result<Json<Todo>, StatusCode> {
    let mut db = DB.lock().await;
    if db.contains_key(&id) {
        db.insert(id, payload.clone());
        Ok(Json(payload))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

//DELETE /todos/:id
async fn delete_todo(Path(id): Path<u32>) -> Result<Json<String>, StatusCode> {
    let mut db = DB.lock().await;
    if db.remove(&id).is_some() {
        Ok(Json(format!("Todo {} deleted", id)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}