use http::Method;
use tres_api::model::ModelManager;
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let mm = ModelManager::new().await;

    match *req.method() {
        Method::GET => tres_api::actions::list_records::handler(&mm).await,

        Method::POST => tres_api::actions::create_record::handler(&mm).await,

        _ => todo!(),
    }
}
