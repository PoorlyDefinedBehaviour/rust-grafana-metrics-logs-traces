use std::{net::SocketAddr, time::Duration};

use axum::{
    extract::Path,
    http::{HeaderName, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use extensions::ExtractContext;
use opentelemetry::KeyValue;

use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod constants;
mod context;
mod extensions;

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::never("./log", "app.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_timeout(Duration::from_secs(5))
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            opentelemetry_sdk::trace::config()
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    env!("CARGO_PKG_NAME"),
                )])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("creating exporter");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();

    let app = Router::new()
        .route("/users/:id", get(handler))
        .route_layer(SetRequestIdLayer::new(
            HeaderName::from_static(constants::REQUEST_ID),
            MakeRequestUuid::default(),
        ))
        // propagate `x-request-id` headers from request to response
        .route_layer(PropagateRequestIdLayer::new(HeaderName::from_static(
            constants::REQUEST_ID,
        )));

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();

    info!(?addr, "starting server");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("running http server");
}

#[tracing::instrument(name="GET /users/:id", skip_all, fields(
  request_id = %ctx.request_id,
  user_id = %id
))]
async fn handler(ExtractContext(ctx): ExtractContext, Path(id): Path<u64>) -> Response {
    match get_user_by_id(id).await {
        Ok(user) => match user {
            None => (StatusCode::NOT_FOUND, StatusCode::NOT_FOUND.as_str()).into_response(),
            Some(user) => Json(user).into_response(),
        },
        Err(err) => {
            let response = (
                StatusCode::INTERNAL_SERVER_ERROR,
                StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            )
                .into_response();

            error!(?err, ?id, ?response, "fetching user by id");

            response
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct User {
    id: u64,
    name: String,
}

#[derive(Debug)]
struct QueryTimeoutError;

#[tracing::instrument(name="get_user_by_id", skip_all, fields(
  user_id = %id
))]
async fn get_user_by_id(id: u64) -> Result<Option<User>, QueryTimeoutError> {
    match id {
        1 => Ok(Some(User {
            id: 1,
            name: "bob".to_owned(),
        })),
        2 => Ok(Some(User {
            id: 1,
            name: "john".to_owned(),
        })),
        3 => Ok(None),
        _ => Err(QueryTimeoutError),
    }
}
