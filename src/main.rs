use axum::{Json, Router,extract::{Path }, http::StatusCode, response::IntoResponse, routing::{get,post}};
use serde_json::{json,Value};
use sqlx::{ Executor, sqlite::{SqliteConnectOptions, SqlitePool}};
use serde::Deserialize;

#[derive(Debug)]
enum APIError{
    NotFound,
    InternalError,
}

#[derive(Deserialize)]
struct DareInfo{
    dare:String,
    created_by:String,
}
#[derive(Deserialize,sqlx::FromRow)]
struct DareRetrived{
    dare:String,
    #[sqlx(rename="createdBy")]
    created_by:String
}
impl IntoResponse for APIError{
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            APIError::NotFound => (
                StatusCode::NOT_FOUND, "Requested data not found".to_string(),
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
    Json(json!({"status": "ok", "message": "Server up",}))
}
async fn add_dare(Json(payload): Json<DareInfo>) -> Result<Json<Value>,APIError>{
    let option = SqliteConnectOptions::new().filename("dares.db").create_if_missing(true);
    let connection = SqlitePool::connect_with(option).await.unwrap();
    connection.execute("
        CREATE TABLE IF NOT EXISTS dares (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dare TEXT,
            createdBy Text
        )
    ").await.unwrap();
    sqlx::query("INSERT INTO dares (dare,createdBy) VALUES (?1,?2)").bind(&payload.dare).bind(&payload.created_by).execute(&connection).await.map_err(|_| APIError::InternalError)?;
    Ok(Json(json!({"status": "ok", "message": "dare added!",})))
}
async fn find_dare(Path(id): Path<i64>) -> Result<Json<Value>,APIError>{

    let option = SqliteConnectOptions::new().filename("dares.db").create_if_missing(true);
    let connection = SqlitePool::connect_with(option).await.unwrap();
    connection.execute("
        CREATE TABLE IF NOT EXISTS dares (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dare TEXT,
            createdBy Text
        )
    ").await.unwrap();
    let row = sqlx::query_as::<_,DareRetrived>("SELECT dare, createdBy from dares WHERE id = ?1").bind(id).fetch_optional(&connection).await.map_err(|_| APIError::InternalError)?;
    match row {
        Some(r) =>  Ok( Json( json!({"id": id,"dare": r.dare, "created_by": r.created_by}) )),
        None => Err(APIError::NotFound),
    }
}
async fn random_dare() -> Result<Json<Value>,APIError>{

    let option = SqliteConnectOptions::new().filename("dares.db").create_if_missing(true);
    let connection = SqlitePool::connect_with(option).await.unwrap();
    connection.execute("
        CREATE TABLE IF NOT EXISTS dares (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dare TEXT,
            createdBy Text
        )
    ").await.unwrap();
    let row = sqlx::query_as::<_,DareRetrived>("SELECT * FROM dares ORDER BY RANDOM() LIMIT 1").fetch_optional(&connection).await.map_err(|_| APIError::InternalError)?;
    match row {
        Some(r) =>  Ok( Json( json!({"dare": r.dare, "created_by": r.created_by}) )),
        None => Err(APIError::NotFound),
    }
}
fn create_app() -> Router{
    Router::new().route("/health", get(health_check)).route("/add", post(add_dare)).route("/find/{id}", get(find_dare)).route("/random", get(random_dare))
}
#[tokio::main]
async fn main() {

    let app = create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("TCP Listener not bound");
    println!("Server live on port 3000");
    axum::serve(listener, app).await.expect("Server start failed");
}
