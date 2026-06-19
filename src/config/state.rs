use std::sync::Arc;

use anyhow::Result;

use crate::config::{app::AppConfig, database::DbManager};

#[derive(Clone)]
pub struct AppState {
    pub db: DbManager,
    pub config: AppConfig,
    pub jwt_secret: String,
}

impl AppState {
    pub async fn new(config: &AppConfig) -> Result<Arc<Self>> {
        let db = DbManager::new().await?;
        let app_state = AppState {
            config: config.clone(),
            db,
            jwt_secret: config.jwt_secret.clone(),
        };

        Ok(Arc::new(app_state))
    }
}
