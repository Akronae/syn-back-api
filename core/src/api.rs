use actix_cors::Cors;

use actix_web::{
    web::{self},
    App, HttpServer,
};

use tracing_actix_web::TracingLogger;

pub mod lexicon;
pub mod verse;

use crate::{
    api::{
        lexicon::{lexicon_controller, lexicon_repo},
        verse::{verse_controller, verse_repo},
    },
    config::EnvVar,
    error::{MapErrSafe, SafeError},
};

pub fn cors() -> Cors {
    Cors::default()
        .allow_any_header()
        .allow_any_method()
        .allow_any_origin()
}

pub async fn init() -> Result<(), SafeError> {
    let port = EnvVar::Port.get()?;

    tracing::info!(
        address = format!("http://localhost:{}", port),
        "API listening on"
    );

    verse_repo::configure().await?;
    lexicon_repo::configure().await?;

    HttpServer::new(|| {
        let routes = web::scope("v1")
            .configure(verse_controller::configure)
            .configure(lexicon_controller::configure);

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors())
            .service(routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
    .map_err_safe()
}
