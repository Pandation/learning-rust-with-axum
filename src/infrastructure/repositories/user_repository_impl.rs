use crate::api::domains::user::entities::User;
use crate::api::domains::user::repository::UserRepository;

impl UserRepositoryImpl {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn create_user(&self, user: User) -> Result<User, Box<dyn std::error::Error>> {
        self.users.push(user);
        Ok(user)
    }

    fn get_user(&self, id: String) -> Result<User, Box<dyn std::error::Error>> {
        self.users.iter().find(|user| user.id == id).ok_or(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found")))
    }

    fn update_user(&self, id: String, user: User) -> Result<User, Box<dyn std::error::Error>> {
        self.users.iter_mut().find(|user| user.id == id).ok_or(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found")))
    }
    
    fn delete_user(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        self.users.iter().find(|user| user.id == id).ok_or(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found")))
    }
}