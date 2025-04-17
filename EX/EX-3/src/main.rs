use EX_3::core::dto::user::User;
use EX_3::core::traits::create_user::CreateUser;
use EX_3::core::traits::find_all_user::FindAllUser;
use EX_3::core::util::check_empty;
// use EX_3::core::config::settings

// ニュータイプを定義
pub struct MyUser(pub User);

impl CreateUser for MyUser {
    fn create_user(&self, name: String, email: String, password: String) -> Result<User, String> {
        if check_empty::is_empty(&name) || check_empty::is_empty(&email) || check_empty::is_empty(&password) {
            return Err("Fields cannot be empty".to_string());
        }
        Ok(User {
            id: 1,
            name,
            email,
            password,
        })
    }
}

impl FindAllUser for MyUser {
    fn find_all_user(&self) -> Result<Vec<User>, String> {
        // ここでは仮のデータを返す例を示します
        // 実際のアプリケーションではデータベースやAPIから取得するでしょう
        let mut users = Vec::new();
        
        // テスト用のユーザーデータを追加
        users.push(User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            password: "password123".to_string(),
        });
        
        users.push(User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            password: "password456".to_string(),
        });
        
        // ユーザーが見つからない場合はエラーを返す
        if users.is_empty() {
            return Err("No users found".to_string());
        }
        
        Ok(users)
    }
}


fn main() {
    // 初期ユーザーを作成
    let default_user = User {
        id: 0,
        name: "Default".to_string(),
        email: "default@example.com".to_string(),
        password: "default123".to_string(),
    };
    
    // ニュータイプでラップ
    let my_user = MyUser(default_user);
    
    // 新しいユーザーを作成
    match my_user.create_user(
        "John Doe".to_string(),
        "john@example.com".to_string(),
        "secure123".to_string()
    ) {
        Ok(user) => {
            println!("ユーザーが作成されました:");
            println!("ID: {}", user.id);
            println!("名前: {}", user.name);
            println!("メール: {}", user.email);
        },
        Err(err) => {
            println!("ユーザー作成エラー: {}", err);
        }
    }
    
    // 全ユーザーを取得
    match my_user.find_all_user() {
        Ok(users) => {
            println!("\n全てのユーザー:");
            for user in users {
                println!("ID: {}, 名前: {}, メール: {}", user.id, user.name, user.email);
            }
        },
        Err(err) => {
            println!("ユーザー取得エラー: {}", err);
        }
    }
    
    // 空のフィールドでユーザーを作成しようとする（エラーになるはず）
    match my_user.create_user(
        "".to_string(),
        "invalid@example.com".to_string(),
        "password".to_string()
    ) {
        Ok(_) => {
            println!("\n意図しない成功: 空の名前でユーザーが作成されました");
        },
        Err(err) => {
            println!("\n期待されるエラー: {}", err);
        }
    }
}
