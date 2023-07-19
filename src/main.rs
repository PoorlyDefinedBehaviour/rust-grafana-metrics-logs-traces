use axum::{response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("running http server");
}

async fn handler() -> impl IntoResponse {
    "hello world"
}
