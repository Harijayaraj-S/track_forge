use std::sync::Arc;

use axum::Extension;

use crate::config::state::AppState;

pub type ExtAppState = Extension<Arc<AppState>>;
