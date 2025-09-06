use self::entities::User;

pub trait UserRepository {
    fn create_user(&self, user: User) -> Result<User, Box<dyn std::error::Error>>;
    fn get_user(&self, id: String) -> Result<User, Box<dyn std::error::Error>>;
    fn update_user(&self, id: String, user: User) -> Result<User, Box<dyn std::error::Error>>;
    fn delete_user(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
}