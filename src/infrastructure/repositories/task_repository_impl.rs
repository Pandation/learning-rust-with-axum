use sqlx::{Pool, Postgres, Row};
use crate::domain::task::entities::Task;
use crate::domain::task::task_repository::TaskRepository;

pub struct TaskRepositoryImpl {
    pool: Pool<Postgres>,
}

impl TaskRepositoryImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl TaskRepository for TaskRepositoryImpl {
    async fn create_task(&self, task: Task) -> Result<Task, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query(r#"
            INSERT INTO tasks (id, title, description, completed)
            VALUES ($1, $2, $3, $4)
        "#)
        .bind(&task.id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(task.completed)
        .execute(&self.pool)
        .await?;
        
        Ok(task)
    }

    async fn get_all_tasks(&self) -> Result<Vec<Task>, Box<dyn std::error::Error + Send + Sync>> {
        let rows = sqlx::query(r#"
            SELECT id, title, description, completed
            FROM tasks
        "#)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows.into_iter().map(|row| Task {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            completed: row.get("completed"),
        }).collect())
    }
    async fn get_task(&self, id: String) -> Result<Task, Box<dyn std::error::Error + Send + Sync>> {
        let row = sqlx::query(r#"
            SELECT id, title, description, completed
            FROM tasks
            WHERE id = $1
        "#)
        .bind(&id)
        .fetch_one(&self.pool)
        .await?;
        
        let task = Task {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            completed: row.get("completed"),
        };
        
        Ok(task)
    }

    async fn update_task(&self, id: String, task: Task) -> Result<Task, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query(r#"
            UPDATE tasks
            SET title = $2, description = $3, completed = $4
            WHERE id = $1
        "#)
        .bind(&id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(task.completed)
        .execute(&self.pool)
        .await?;
        
        Ok(task)
    }

    async fn delete_task(&self, id: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query(r#"
            DELETE FROM tasks
            WHERE id = $1
        "#)
        .bind(&id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}