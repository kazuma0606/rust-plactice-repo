
#[derive(Debug, Clone, PartialEq, Default)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: String, age: u8) -> Self {
        return User { name, age }
    }
}

fn main() {
    let user1 = User::new("Alice".to_string(), 30);
    let mut user2 = user1.clone(); // Cloneを使ってuser1を複製

    println!("User1: {:?}", user1);
    println!("User2: {:?}", user2);

    user2.name = "Bob".to_string(); // user2の名前を変更
    user2.age = 25; // user2の年齢を変更

    println!("User1 after modifying User2: {:?}", user1);
    println!("User2 after modification: {:?}", user2);
    // user1とuser2の比較
    if user1 == user2 {
        println!("User1 and User2 are equal.");
    } else {
        println!("User1 and User2 are not equal.");
    }

    // user1のデフォルト値を使用
    let user3: User = Default::default();
    println!("User3 (default): {:?}", user3);
}
