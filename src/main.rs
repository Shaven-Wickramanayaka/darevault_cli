
use axum::{Json, Router,extract::Query, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::{json,Value};
use sqlx::{Connection, Executor, prelude::FromRow, sqlite::{SqliteConnectOptions, SqlitePool}};
use serde::Deserialize;

#[derive(Debug)]
enum APIError{
    NotFound,
    InvalidInput(String),
    InternalError,
}

#[derive(Deserialize)]
struct DareInfo{
    dare:String,
    name:String,
}
impl IntoResponse for APIError{
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            APIError::NotFound => (
                StatusCode::NOT_FOUND, "Requested data not found".to_string(),
            ),
            APIError::InvalidInput(msg)=> (
                StatusCode::BAD_REQUEST,msg,
            ),
            APIError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR, "Server error".to_string(),
            ),
        };
        let body =  Json(json!({"error":error_message}));
        (status, body).into_response()
    }
}
async fn health_check() -> impl IntoResponse{
    Json(json!({"status": "ok", "message": "Server up!",}))
}
async fn add_dare(Query(params): Query<DareInfo>) -> Result<Json<Value>,APIError>{
    let option = SqliteConnectOptions::new().filename("dares.db").create_if_missing(true);
    let connection = SqlitePool::connect_with(option).await.unwrap();
    connection.execute("
        CREATE TABLE IF NOT EXISTS dares (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dare TEXT,¬
            createdBy Text
        )
    ").await.unwrap();
    let dare = params.dare;
    let name = params.name;
    sqlx::query("INSERT INTO dares (dare,createdBy) VALUES (?1,?2)").bind(dare).bind(name).execute(&connection).await.map_err(|_| APIError::InternalError)?;
    Ok(Json(json!({"status": "ok", "message": "dare added!",})))
}
fn create_app() -> Router{
    Router::new().route("/health", get(health_check)).route("/add", get(add_dare))
}
#[tokio::main]
async fn main() {
    // let option = SqliteConnectOptions::new().filename("dares.db").create_if_missing(true);
    // let connection = SqlitePool::connect_with(option).await.unwrap();
    // connection.execute("
    //     CREATE TABLE IF NOT EXISTS dares (
    //         id INTEGER PRIMARY KEY AUTOINCREMENT,
    //         dare TEXT,
    //         createdBy Text
    //     )
    // ").await.unwrap();
    let app = create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("TCP Listener not bound");
    println!("Server live on port 3000");
    axum::serve(listener, app).await.expect("Server start failed");
}
