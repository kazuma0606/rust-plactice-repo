#![allow(dead_code)]

use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use axum::{
    Router,
    routing::get,
    extract::{State, Path},
    response::IntoResponse,
};
use std::sync::OnceLock;

// グローバル設定
static CONFIG: OnceLock<String> = OnceLock::new();

fn init_config() {
    CONFIG.set("production".to_string()).unwrap();
}

fn get_config() -> &'static str {
    CONFIG.get().map(|s| s.as_str()).unwrap_or("unknown")
}

// type alias
// DBはHashMap<u32, String>をラップしたMutexを指す
type DB = Arc<Mutex<HashMap<u32, String>>>;

// データアクセス層のインターフェース
#[async_trait::async_trait]
trait DataAccess: Clone + Send + Sync {
    async fn get_count(&self) -> usize;
    async fn get_item(&self, id: u32) -> Option<String>;
}

// 実際のDB実装
#[derive(Clone)]
struct DatabaseAccess {
    db: DB,
}

#[async_trait::async_trait]
impl DataAccess for DatabaseAccess {
    async fn get_count(&self) -> usize {
        let db = self.db.lock().await;
        db.len()
    }
    
    async fn get_item(&self, id: u32) -> Option<String> {
        let db = self.db.lock().await;
        db.get(&id).cloned()
    }
}

// モックデータアクセス（説明用）
#[derive(Clone)]
struct MockDataAccess {
    fixed_count: usize,
}

#[async_trait::async_trait]
impl DataAccess for MockDataAccess {
    async fn get_count(&self) -> usize {
        self.fixed_count
    }
    
    async fn get_item(&self, _id: u32) -> Option<String> {
        Some("モックデータ".to_string())
    }
}

// ハンドラー関数
async fn handler<T: DataAccess>(State(state): State<AppState<T>>) -> String {
    let count = state.data_access.get_count().await;
    format!("要素数: {}", count)
}

async fn get_item<T: DataAccess>(
    State(state): State<AppState<T>>, 
    Path(id): Path<u32>
) -> impl IntoResponse {
    match state.data_access.get_item(id).await {
        Some(item) => item,
        None => "アイテムが見つかりません".to_string(),
    }
}

async fn info<T: DataAccess>(State(state): State<AppState<T>>) -> String {
    format!("アプリ名: {}", state.app_name)
}

// アプリケーション状態 - `Clone` を実装
#[derive(Clone)]
struct AppState<T: DataAccess> {
    data_access: T,
    app_name: String,
}

// DIを適用したルーター設定（修正版）
fn app<T: DataAccess + 'static>(data_access: T, app_name: String) -> Router {
    let app_state = AppState {
        data_access,
        app_name,
    };
    
    Router::new()
        .route("/", get(handler::<T>))
        .route("/info", get(info::<T>))
        .route("/items/{id}", get(get_item::<T>))
        .with_state(app_state)
}

#[tokio::main]
async fn main() {
    // 設定を初期化
    init_config();
    
    // DBの初期化
    let db: DB = Arc::new(Mutex::new(HashMap::new()));
    
    // 初期データを追加
    {
        let mut db_lock = db.lock().await;
        db_lock.insert(1, "項目1".to_string());
        db_lock.insert(2, "項目2".to_string());
    }
    
    // データアクセスレイヤーを作成
    let data_access = DatabaseAccess { db };
    
    // アプリケーションを構築して実行
    let app = app(data_access, "本番アプリ".to_string());
    
    // サーバーを起動
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("サーバーを起動します: {}", addr);
    println!("環境: {}", get_config());
    

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    
    // 説明用: モックを使う場合はこうなる
    // let mock_data = MockDataAccess { fixed_count: 10 };
    // let test_app = app(mock_data, "テストアプリ".to_string());
}