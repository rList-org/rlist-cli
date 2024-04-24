use std::sync::Arc;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use rlist_vfs::Wheel;

pub async fn handler(
    Path(path): Path<String>,
    State(state): State<Arc<Wheel>>,
) -> impl IntoResponse {
    let map = state.path_map.read();
    match map.get(&path) {
        Some(file) => {
            Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", file.random_link())
                .body("Redirecting".to_string())
                .unwrap()
        },
        None => {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Resource not found".into())
                .unwrap()
        },
    }
}