use axum::{routing::post, Json, Router};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
struct LogEntry {
    service: String,
    level: String,
    message: String,
}

type LogStore = Arc<Mutex<Vec<String>>>;

#[tokio::main]
async fn main() {
    let logs: LogStore = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/log", post(accept_log))
        .with_state(logs.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4200));
    println!("📝 Logging Service running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_state(logs))
        .await
        .unwrap();
}

async fn accept_log(
    Json(entry): Json<LogEntry>,
    state: axum::extract::State<LogStore>,
) -> Json<serde_json::Value> {
    let timestamp = Utc::now().to_rfc3339();
    let log_line = format!(
        "[{}] [{}] {}: {}",
        timestamp, entry.level, entry.service, entry.message
    );

    let mut store = state.lock().await;
    store.push(log_line.clone());

    Json(json!({ "status": "logged", "entry": log_line }))
}
