use crate::core::dto::user::User;

pub trait CreateUser {
    fn create_user(&self, name: String, email: String, password: String) -> Result<User, String>;
}