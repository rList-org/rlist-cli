use std::sync::Arc;
use axum::extract::State;
use axum::http::{header, HeaderValue};
use axum::response::{IntoResponse, Response};
use rlist_vfs::Wheel;

pub async fn handler(
    State(state): State<Arc<Wheel>>,
) -> impl IntoResponse {
    let data= state.tree.read().as_ref().clone();
    let mut response = Response::new(data);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    response
}
