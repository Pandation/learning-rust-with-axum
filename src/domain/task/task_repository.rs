use self::entities::Task;

pub trait TaskRepository {
    fn create_task(&self, task: Task) -> Result<Task, Box<dyn std::error::Error>>;
    fn get_task(&self, id: String) -> Result<Task, Box<dyn std::error::Error>>;
    fn update_task(&self, id: String, task: Task) -> Result<Task, Box<dyn std::error::Error>>;
    fn delete_task(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
}