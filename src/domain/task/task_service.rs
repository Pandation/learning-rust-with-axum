use self::entities::Task;

pub struct TaskService ;

impl TaskService for TaskService {
    fn is_completed(&self, task: &Task) -> bool {
        task.completed
    }
}