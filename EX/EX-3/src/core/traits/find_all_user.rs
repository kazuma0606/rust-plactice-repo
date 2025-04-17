use crate::core::dto::user::User;

pub trait FindAllUser {
    fn find_all_user(&self) -> Result<Vec<User>, String>;
}