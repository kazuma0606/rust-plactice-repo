//collection
use std::collections::HashMap;

fn main() {
    //vector
    let mut numbers = vec![1, 2, 3, 4];
    numbers.push(5);
    println!("{:?}", numbers);


    //HashMap
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 60);

    println!("{:?}", scores);


    //Json like data
    let mut user_data = HashMap::new();
    user_data.insert("name", "Jhon");
    user_data.insert("age", "30");
    user_data.insert("email", "example@mail.com");

    let mut user_repository = HashMap::new();
    user_repository.insert("user1", user_data);

    // println!("{:?}", user_repository);
    // {"user1": {"age": "30", "email": "example@mail.com", "name": "Jhon"}}

    // Convert to JSON
    // To use serde_json, add the following to your Cargo.toml:
    let json = serde_json::to_string_pretty(&user_repository).unwrap();
    println!("{}", json);
}
