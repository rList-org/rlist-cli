use std::sync::Arc;
use axum::routing::get;
use rlist_vfs::Wheel;
use crate::state::AppState;

mod profile;

#[inline]
pub fn routes(app_state: Arc<AppState>, wheel: Arc<Wheel>) -> axum::Router {
    axum::Router::new()
        .route("/profile", get(profile::handler))
        .with_state(app_state)
}