use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(id: String, first_name: String, last_name: String, email: String, password: String) -> Self {
        Self { id, first_name, last_name, email, password }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
    
}