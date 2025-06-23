use backend::{config::Config, http};
use dotenvy::dotenv;
use envconfig::Envconfig;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::init_from_env()?;
    let router = http::new_router();
    let listener = TcpListener::bind(&config.http_address).await?;

    log::info!("running http server on {}", &config.http_address);

    axum::serve(listener, router).await?;

    Ok(())
}
