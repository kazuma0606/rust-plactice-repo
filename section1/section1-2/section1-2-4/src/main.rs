struct User {
    name: String,
    age: u32,
    role: Role,
    status: Status,
}

enum Role {
    Admin,
    User,
}
enum Status {
    Active,
    Inactive,
}

impl User {
    fn new(name: String, age: u32, role: Role, status: Status) -> User {
        User { name, age, role, status }
    }
    fn check_role(&self) {
        match self.role {
            Role::Admin => println!("{} is an admin", self.name),
            Role::User => println!("{} is a user", self.name),
        }
    }
    fn check_status(&self) {
        match self.status {
            Status::Active => println!("{} is active", self.name),
            Status::Inactive => println!("{} is inactive", self.name),
        }
    }
}

fn main() {
    // let user = User {
    //     name: String::from("Alice"),
    //     age: 30,
    // };

    // println!("{} is {} years old", user.name, user.age);

    //new instance
    let user = User::new(
        String::from("Alice"),
        30,
        Role::Admin,
        Status::Active,
    );

    println!("{} is {} years old", user.name, user.age);

    let _ = User::new(
        String::from("_"),
        0,
        Role::User,
        Status::Inactive,
    );

    // match user.role {
    //     Role::Admin => println!("{} is an admin", user.name),
    //     Role::User => println!("{} is a user", user.name),
    // }
    // match user.status {
    //     Status::Active => println!("{} is active", user.name),
    //     Status::Inactive => println!("{} is inactive", user.name),
    // }

    user.check_role();
    user.check_status();
}
