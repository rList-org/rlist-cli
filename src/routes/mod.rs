use std::sync::Arc;
use axum::routing::get;
use rlist_vfs::Wheel;
use crate::state::AppState;

mod profile;
mod tree;
mod download_link;

#[inline]
pub fn routes(app_state: Arc<AppState>, wheel: Arc<Wheel>) -> axum::Router {
    axum::Router::new()
        .route("/profile", get(profile::handler))
        .with_state(app_state)
        .route("/tree", get(tree::handler))
        .with_state(wheel.clone())
        .route("/download/*path", get(download_link::handler))
        .with_state(wheel.clone())
}