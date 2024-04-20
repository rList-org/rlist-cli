use std::sync::Arc;
use axum::routing::get;
use crate::state::AppState;

mod profile;

#[inline]
pub fn routes(app_state: Arc<AppState>) -> axum::Router {
    axum::Router::new()
        .route("/profile", get(profile::handler))
        .with_state(app_state)
}