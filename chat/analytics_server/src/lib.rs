mod config;
mod error;
mod events;
mod extractors;
mod handler;
mod openapi;
pub mod pb;

use anyhow::Context;
use axum::{http::Method, middleware::from_fn_with_state, routing::post, Router};
pub use config::*;
pub use error::*;
use handler::*;
use openapi::OpenApiRouter as _;
use tokio::fs;
use tower_http::cors::{self, CorsLayer};

use std::{fmt, ops::Deref, sync::Arc};

use chat_core::{
    middlewares::{extract_user, TokenVerify},
    DecodingKey, User,
};
use clickhouse::Client;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(dead_code)]
pub struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) dk: DecodingKey,
    pub(crate) client: Client,
}

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_headers(cors::Any)
        .allow_origin(cors::Any);

    let api = Router::new()
        .route("/event", post(create_event_handler))
        .layer(from_fn_with_state(state.clone(), extract_user::<AppState>))
        .layer(cors);

    let app = Router::new().openapi().nest("/api", api).with_state(state);
    Ok(app)
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        fs::create_dir_all(&config.server.base_dir)
            .await
            .context("create base_dir failed")?;
        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let mut client = Client::default()
            .with_url(&config.server.db_url)
            .with_database(&config.server.db_name);
        if let Some(db_user) = &config.server.db_user {
            client = client.with_user(db_user)
        }
        if let Some(db_password) = &config.server.db_password {
            client = client.with_password(db_password);
        }
        Ok(AppState {
            inner: Arc::new(AppStateInner { config, dk, client }),
        })
    }
}

impl TokenVerify for AppState {
    type Error = AppError;

    fn verify(&self, token: &str) -> Result<User, Self::Error> {
        Ok(self.dk.verify(token)?)
    }
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}
