use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: String, title: String, description: String, completed: bool) -> Self {
        Self { id, title, description, completed }
    }
}

impl Task {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
    
}


pub struct Comment {
    pub id: String,
    pub content: String,
    pub task_id: String,
    pub created_at: DateTime<Utc>,
}

impl Comment {
    pub fn new(id: String, content: String, task_id: String) -> Self {
        Self { id, content, task_id, created_at: Utc::now() }
    }
}