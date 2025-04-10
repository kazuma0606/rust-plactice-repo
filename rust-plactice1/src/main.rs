use validator::{
    Validate,
    // ValidationErrors,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
struct RegistUser{
    #[validate(length(min = 1, max = 20))]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(range(min = 18, max = 100))]
    age: u8,
}

fn main() {
    let user = RegistUser {
        name: "John Doe".to_string(),
        email: "example@test.com".to_string(),
        age: 25,
    };

    match user.validate() {
        Ok(_) => println!("validation passed"),
        Err(err) => println!("validation failed: {:?}",err),
    }
}
