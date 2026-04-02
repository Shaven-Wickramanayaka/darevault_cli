use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;
#[derive(Debug)]
enum APIError{
    NotFound,
    InvalidInput(String),
    InternalError,
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
fn create_app() -> Router{
    Router::new().route("/health", get(health_check))
}
#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("TCP Listener not bound");
    println!("Server live on port 3000");
    axum::serve(listener, app).await.expect("Server start failed");
}
