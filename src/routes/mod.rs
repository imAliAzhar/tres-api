mod create_record;
mod list_records;

use axum::{
    routing::{get, post},
    Router,
};

use super::ModelManager;

const PATH: &str = "/records";

pub fn router(model_manager: ModelManager) -> Router {
    Router::new()
        .route(PATH, post(create_record::handler))
        .route(PATH, get(list_records::handler))
        .with_state(model_manager)
}
