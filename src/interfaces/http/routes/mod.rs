mod task_routes;

use axum::Router;
use crate::infrastructure::app_state::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/tasks", task_routes::create_routes())
}
