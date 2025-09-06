use self::entities::User;

pub struct UserService;

impl UserService for UserService {
    fn get_full_name(&self, user: &User) -> String {
        user.first_name.clone() + " " + user.last_name.clone()
    }
}