use axum::response::Html;

use crate::config::types::ExtAppState;

pub async fn handler(state: ExtAppState) -> Html<String> {
    state.db.health_check().await.unwrap();

    Html(format!(
        "<h1>Track Forge server is running in {:?} Env</h1>",
        state.config.env
    ))
}
