use super::entities::Task;
use std::future::Future;

pub trait TaskRepository {
    fn create_task(&self, task: Task) -> impl Future<Output = Result<Task, Box<dyn std::error::Error + Send + Sync>>>;
    fn get_task(&self, id: String) -> impl Future<Output = Result<Task, Box<dyn std::error::Error + Send + Sync>>>;
    fn update_task(&self, id: String, task: Task) -> impl Future<Output = Result<Task, Box<dyn std::error::Error + Send + Sync>>>;
    fn delete_task(&self, id: String) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
    fn get_all_tasks(&self) -> impl Future<Output = Result<Vec<Task>, Box<dyn std::error::Error + Send + Sync>>>;
}