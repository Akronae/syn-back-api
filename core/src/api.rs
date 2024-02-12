use std::io;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{
    http,
    web::{self},
    App, HttpRequest, HttpServer,
};

use paperclip::actix::OpenApiExt;
use tracing_actix_web::TracingLogger;

mod verse;

use crate::{
    api::verse::verse_controller,
    config::{Config, EnvVar},
    error::{MapErrIo, MapIntoErr},
};

pub fn cors() -> Cors {
    return Cors::default()
        .allow_any_header()
        .allow_any_method()
        .allow_any_origin();
}

pub async fn init() -> std::io::Result<()> {
    let port = Config.get(EnvVar::Port).map_err_io()?;

    tracing::info!(
        address = format!("http://localhost:{}", port),
        "API listening on"
    );

    HttpServer::new(|| {
        let routes = web::scope("v1").configure(verse_controller::configure);
        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors())
            .service(routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
