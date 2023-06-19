use futures::{future::BoxFuture, FutureExt};
use springtime::application::{self, Application};
use springtime_di::{
    component_alias, factory::ComponentFactory, instance_provider::ErrorPtr, Component,
};
use springtime_web_axum::config::{
    ServerConfig, WebConfig, WebConfigProvider, DEFAULT_SERVER_NAME,
};

use crate::{
    config::{Config, EnvVar},
    error::SafeError,
};

use tracing::info;

pub mod verse;

#[derive(Component)]
#[component(constructor = "MyWebConfigProvider::new")]
struct MyWebConfigProvider {
    #[component(ignore)]
    config: WebConfig,
}

impl MyWebConfigProvider {
    fn new() -> BoxFuture<'static, Result<Self, ErrorPtr>> {
        async {
            let mut web_config = WebConfig::default();
            let mut server_config = ServerConfig::default();

            server_config.listen_address =
                format!("127.0.0.1:{}", Config.get_i32(EnvVar::Port).unwrap());

            info!(addr = server_config.listen_address, "listening");

            web_config
                .servers
                .insert(DEFAULT_SERVER_NAME.to_string(), server_config);

            Ok(Self { config: web_config })
        }
        .boxed()
    }
}

#[component_alias]
impl WebConfigProvider for MyWebConfigProvider {
    fn config(&self) -> BoxFuture<'_, Result<&WebConfig, ErrorPtr>> {
        async { Ok(&self.config) }.boxed()
    }
}

pub async fn init() -> Result<Application<ComponentFactory>, SafeError> {
    let mut app = application::create_default()?;
    app.run().await?;

    Ok(app)
}
