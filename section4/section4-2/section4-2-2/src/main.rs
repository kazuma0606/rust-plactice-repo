struct NameSet;
struct AgeSet;
struct NotSet;

struct UserBuilder<Name = NotSet, Age = NotSet> {
    name: Option<String>,
    age: Option<u32>,
    _marker: std::marker::PhantomData<(Name, Age)>,
}

impl UserBuilder {
    fn new() -> Self {
        UserBuilder {
            name: None,
            age: None,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<Age> UserBuilder<NotSet, Age> {
    fn name(self, name: String) -> UserBuilder<NameSet, Age> {
        UserBuilder {
            name: Some(name),
            age: self.age,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<Name> UserBuilder<Name, NotSet> {
    fn age(self, age: u32) -> UserBuilder<Name, AgeSet> {
        UserBuilder {
            name: self.name,
            age: Some(age),
            _marker: std::marker::PhantomData,
        }
    }
}

struct User {
    name: String,
    age: u32,
}

impl UserBuilder<NameSet, AgeSet> {
    fn build(self) -> User {
        User {
            name: self.name.unwrap(),
            age: self.age.unwrap(),
        }
    }
}

fn main() {
    let user = UserBuilder::new()
        .name("Alice".to_string())
        .age(30)
        .build();

    println!("User: {} is {} years old", user.name, user.age);
}
