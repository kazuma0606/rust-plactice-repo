use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IDが不正です。")]
    InvlidId,
    #[error("タイトルは空にできません。")]
    EmptyTitle,
}

struct  BlogPost {
    id: u32,
    title: String,
}
impl BlogPost {
    fn new(id: u32, title: String) -> Result<Self, AppError> {
        if id == 0 {
            return Err(AppError::InvlidId);
        }
        if title.is_empty() {
            return Err(AppError::EmptyTitle);
        }
        Ok(BlogPost { id, title })
    }

    //動的ディスパッチを使用してエラーを返す場合
    // fn _new(id: i32, title: String) -> Result<Self, Box<dyn std::error::Error>>{}
}
fn main() {
    let post = BlogPost::new(11235813, "Rust is so Fun!".to_string());
    match post {
        Ok(p) => println!("ブログページが作成されました。id: {}, title: {}", p.id, p.title),
        Err(e) => println!("エラー: {}", e),
    }
}
