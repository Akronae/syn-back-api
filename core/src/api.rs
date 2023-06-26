use actix_web::{
    web::{self},
    App, HttpServer,
};

use tracing_actix_web::TracingLogger;

mod verse;

use crate::{
    api::verse::verse_controller,
    config::{Config, EnvVar},
    error::MapToIoError,
};

pub async fn init() -> std::io::Result<()> {
    let port = Config.get(EnvVar::Port).map_err_to_io()?;

    tracing::info!(
        address = format!("http://localhost:{}", port),
        "API listening on"
    );

    HttpServer::new(|| {
        let routes = web::scope("v1").configure(verse_controller::configure);
        App::new().wrap(TracingLogger::default()).service(routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
