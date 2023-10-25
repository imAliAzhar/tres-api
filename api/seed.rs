use http::Method;
use tres_api::model::ModelManager;
use vercel_runtime::{Body, Error, Request, Response};

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let mm = ModelManager::new().await;

    match *req.method() {
        Method::POST => tres_api::actions::seed_db::handler(&mm).await,

        _ => todo!(),
    }
}
