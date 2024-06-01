use axum::extract::{Extension, Json, Path, Query, Request};
use axum::{routing::get, routing::patch, routing::post, Router};
use log::{error, info};
use tokio::net::TcpListener;

mod handlers;
use handlers::user_handler::register_users;
#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting the application");

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(register_users))
        .route("/users", get(handlers::user_handler::get_users))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on port 3000");

    if let Err(e) = axum::serve(listener, app).await {
        error!("Failed to start the server: {}", e);
    }
}

async fn root() -> &'static str {
    "get root !!"
}

async fn get_foo() -> &'static str {
    "get foo !!"
}

async fn post_foo() -> &'static str {
    "post foo !!"
}

async fn foo_bar() -> &'static str {
    "foo bar !!"
}
