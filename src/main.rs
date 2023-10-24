use axum::Router;

use routes::router;
use sqlx::PgPool;

mod base;
mod error;
mod model;
mod prelude;
mod routes;

pub use error::{Error, Result};

use model::ModelManager;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_aws_rds::Postgres(local_uri = "{secrets.LOCAL_URI}")] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    let model_manager = ModelManager::new(pool);

    let router = Router::new().nest("/api", router(model_manager));

    Ok(router.into())
}
