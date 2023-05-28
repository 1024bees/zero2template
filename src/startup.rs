use super::routes::health_check;

use axum::body::Body;
use axum::http::Request;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::SqlitePool;
use std::future::Future;
use std::net::TcpListener;

use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};

use crate::configuration::{get_configuration, Settings};
use sqlx::sqlite::SqlitePoolOptions;

// We need to define a wrapper type in order to retrieve the URL
// in the `subscribe` handler.

pub fn run(app: Application) -> impl Future<Output = Result<(), hyper::Error>> {
    let listener = app.listener;
    let app = Router::new()
        .route("/health_check", get(health_check))
        .layer(
            // Let's create a tracing span for each request
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                // We get the request id from the extensions
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                // And then we put it along with other information into the `request` span
                tracing::info_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
        )
        // This layer creates a new id for each request and puts it into the request extensions.
        // Note that it should be added after the Trace layer.
        .layer(RequestIdLayer)
        .layer(Extension(app.pool));

    axum::Server::from_tcp(listener)
        .expect("Spawning server from listener failed")
        .serve(app.into_make_service())
}

// A new type to hold the newly built server and its port
pub struct Application {
    port: u16,
    listener: TcpListener,
    pool: SqlitePool,
}
impl Application {
    // We have converted the `build` function into a constructor for
    // `Application`.
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let pool = SqlitePoolOptions::default()
            .connect(format!("sqlite:{}", configuration.application.db_name).as_str())
            .await
            .expect("Failed connecting to the sqlitepool.. SUCKS!");
        // We "save" the bound port in one of `Application`'s fields
        Ok(Self {
            listener,
            port,
            pool,
        })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), hyper::Error> {
        run(self).await
    }
}
