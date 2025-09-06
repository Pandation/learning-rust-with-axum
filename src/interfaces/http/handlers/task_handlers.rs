use axum::{
    Json,
    extract::{
        State,
        Path,
    },
};
use crate::domain::task::{
    entities::Task,
    task_repository::TaskRepository,
};
use crate::infrastructure::repositories::task_repository_impl::TaskRepositoryImpl;
use crate::infrastructure::app_state::AppState;


pub async fn get_all_tasks(State(app_state): State<AppState>) -> Result<Json<Vec<Task>>, String> {
    let task_repository = TaskRepositoryImpl::new(app_state.database);

    match task_repository.get_all_tasks().await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(e) => Err(format!("Erreur lors de la récupération des tâches: {}", e)),
    }
}

pub async fn get_task(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Task>, String> {
    let task_repository = TaskRepositoryImpl::new(app_state.database);

    match task_repository.get_task(id).await {
        Ok(task) => Ok(Json(task)),
        Err(e) => Err(format!("Erreur lors de la récupération de la tâche: {}", e)),
    }
}

pub async fn create_task(
    State(app_state): State<AppState>,
    Json(task): Json<Task>,
) -> Result<Json<Task>, String> {
    let task_repository = TaskRepositoryImpl::new(app_state.database);

    match task_repository.create_task(task).await {
        Ok(task) => Ok(Json(task)),
        Err(e) => Err(format!("Erreur lors de la création de la tâche: {}", e)),
    }
}

pub async fn update_task(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
    Json(task): Json<Task>,
) -> Result<Json<Task>, String> {
    let task_repository = TaskRepositoryImpl::new(app_state.database);
    match task_repository.update_task(id, task).await {
        Ok(task) => Ok(Json(task)),
        Err(e) => Err(format!("Erreur lors de la mise à jour de la tâche: {}", e)),
    }
}

pub async fn delete_task(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Task>, String> {
    let task_repository = TaskRepositoryImpl::new(app_state.database);

    match task_repository.delete_task(id.clone()).await {
        Ok(_) => Ok(Json(Task::new(id.clone(), "".to_string(), "".to_string(), false))),
        Err(e) => Err(format!("Erreur lors de la suppression de la tâche: {}", e)),
    }
}
