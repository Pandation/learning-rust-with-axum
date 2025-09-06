use axum::Router;
use axum::routing::{get, post, put, delete};
use crate::infrastructure::app_state::AppState;
use crate::interfaces::http::handlers::task_handlers;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(task_handlers::get_all_tasks))
        .route("/:id", get(task_handlers::get_task))
        .route("/", post(task_handlers::create_task))
        .route("/:id", put(task_handlers::update_task))
        .route("/:id", delete(task_handlers::delete_task))
}