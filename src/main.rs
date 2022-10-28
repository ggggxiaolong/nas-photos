// #[macro_use]
// extern crate lazy_static;
// extern crate thiserror;

mod api;
mod dao;
mod vo;

use std::sync::Mutex;
use api::Api;
use dao::Config;
use poem::{
    endpoint::StaticFilesEndpoint, listener::TcpListener, middleware::Cors, EndpointExt, Result,
    Route, Server,
};
use poem_openapi::OpenApiService;


type DbPool = sqlx::SqlitePool;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    let pool = DbPool::connect("sqlite://firm.db").await?;
    let api_service = OpenApiService::new(Api, "Photos", "1.0.0").server("http://0.0.0.0:3000");
    let ui = api_service.swagger_ui();
    // let spec = api_service.spec();
    let route = Route::new()
        // .nest("/", StaticFilesEndpoint::new("./dist").index_file("index.html"))
        .nest("/api", api_service)
        .nest("/ui", ui)
        // .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .data(pool);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(route)
        .await?;
    Ok(())
}
