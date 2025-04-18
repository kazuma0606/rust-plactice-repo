#![allow(dead_code)]

use std::marker::PhantomData;

// マーカートレイト
trait Validated {}
trait Unvalidated {}

// 特定の型をマーカートレイトの実装として指定
struct ValidatedTag;
struct UnvalidatedTag;

// マーカートレイトの実装
impl Validated for ValidatedTag {}
impl Unvalidated for UnvalidatedTag {}

// ジェネリック構造体にPhantomDataとマーカートレイトを使用
struct UserData<T> {
    name: String,
    email: String,
    _marker: PhantomData<T>, // 検証状態を表すマーカー
}

// 未検証ユーザー用の実装
impl UserData<UnvalidatedTag> {
    fn new(name: String, email: String) -> Self {
        UserData {
            name,
            email,
            _marker: PhantomData,
        }
    }

    // 未検証から検証済みに変換するメソッド
    fn validate(self) -> Result<UserData<ValidatedTag>, String> {
        // 名前の検証
        if self.name.trim().is_empty() {
            return Err("名前は空にできません".to_string());
        }

        // メールアドレスの簡易検証
        if !self.email.contains('@') {
            return Err("無効なメールアドレスです".to_string());
        }

        // 検証に成功したら、新しい型状態でデータを返す
        Ok(UserData {
            name: self.name,
            email: self.email,
            _marker: PhantomData,
        })
    }
}

// 検証済みユーザー用の実装
impl UserData<ValidatedTag> {
    // 検証済みデータでのみ利用可能なメソッド
    fn send_welcome_email(&self) -> String {
        format!("{}様、ご登録ありがとうございます！", self.name)
    }

    fn get_user_profile(&self) -> String {
        format!("プロフィール - 名前: {}, メール: {}", self.name, self.email)
    }
}

// 汎用メソッド（どちらの状態でも利用可能）
impl<T> UserData<T> {
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    // 未検証ユーザーデータの作成
    let unvalidated_user = UserData::new(
        "山田太郎".to_string(),
        "yamada@example.com".to_string(),
    );
    
    println!("未検証ユーザー名: {}", unvalidated_user.get_name());
    
    // ここではsend_welcome_emailは呼び出せない（コンパイルエラー）
    // println!("{}", unvalidated_user.send_welcome_email());
    
    // ユーザーデータを検証
    match unvalidated_user.validate() {
        Ok(validated_user) => {
            // 検証済みユーザーでのみ利用可能なメソッドを呼び出せる
            println!("{}", validated_user.send_welcome_email());
            println!("{}", validated_user.get_user_profile());
            
            // 汎用メソッドも使える
            println!("検証済みユーザー名: {}", validated_user.get_name());
        },
        Err(error) => {
            println!("検証エラー: {}", error);
        }
    }
    
    // 不正なデータで試す
    let invalid_user = UserData::new(
        "".to_string(),
        "invalid-email".to_string(),
    );
    
    match invalid_user.validate() {
        Ok(_) => println!("予期しない成功"),
        Err(error) => println!("期待されるエラー: {}", error),
    }
}