use std::error::Error;

use axum::{
    Router,
    routing::{get},
};
use dotenvy::dotenv;
use tower_http::trace::TraceLayer;

use crate::{
    core::{app_state::AppState, config::Config, db::init_db},
    handler::{
        create_expense_handler, delete_expense_handler, find_expense_handler,
        list_expenses_handler, update_expense_handler,
    },
};

mod core;
mod dto;
mod handler;
mod model;
mod repository;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::from_env();

    let pool = init_db(&config.database_url).await?;

    let app_state = AppState { db_pool: pool };

    let app = app(app_state);

    let listener = tokio::net::TcpListener::bind(&config.server_addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

fn app(state: AppState) -> Router {
    let expense_routes = Router::new()
        .route("/", get(list_expenses_handler).post(create_expense_handler))
        .route(
            "/{id}",
            get(find_expense_handler)
                .patch(update_expense_handler)
                .delete(delete_expense_handler),
        );
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .nest("/expense", expense_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler")
}
