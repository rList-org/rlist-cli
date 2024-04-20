use std::sync::Arc;
use axum::{
    extract::State,
};
use axum::response::{IntoResponse, Response};
use crate::config::SiteProfile;
use crate::state::AppState;

pub async fn handler(
    State(state): State<Arc<AppState>>,
) -> SiteProfile {
    state.site_profile.clone()
}

impl IntoResponse for SiteProfile {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}