use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: String, title: String, description: String, completed: bool) -> Self {
        Self {
            id,
            title,
            description,
            completed,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub content: String,
    pub task_id: String,
    pub created_at: String,
}

impl Comment {
    pub fn new(id: String, content: String, task_id: String) -> Self {
        Self {
            id,
            content,
            task_id,
            created_at: chrono::Utc::now().to_string(),
        }
    }
}
