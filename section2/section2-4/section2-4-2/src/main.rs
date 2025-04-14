use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use reqwest::StatusCode;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Todo CLIクライアント", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// すべてのTodoを一覧表示
    List,
    
    /// 特定のIDのTodoを表示
    Get {
        /// 取得するTodoのID
        id: u32,
    },
    
    /// 新しいTodoを作成
    Create {
        /// TodoのID
        id: u32,
        /// Todoのタイトル
        title: String,
    },
    
    /// 既存のTodoを更新
    Update {
        /// 更新するTodoのID
        id: u32,
        /// 新しいタイトル
        title: String,
        /// 完了状態
        #[arg(short, long)]
        completed: bool,
    },
    
    /// Todoを削除
    Delete {
        /// 削除するTodoのID
        id: u32,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();
    
    match cli.command {
        Commands::List => {
            let res = reqwest::get("http://localhost:3000/todos")
                .await?
                .json::<Vec<Todo>>()
                .await?;
            
            if res.is_empty() {
                println!("Todoはありません");
            } else {
                println!("ID | 状態 | タイトル");
                println!("------------------------");
                for todo in res {
                    let status = if todo.completed { "✓" } else { " " };
                    println!("{:2} | [{}] | {}", todo.id, status, todo.title);
                }
            }
        },
        
        Commands::Get { id } => {
            let url = format!("http://localhost:3000/todos/{}", id);
            let res = client.get(&url).send().await?;
            
            match res.status() {
                StatusCode::OK => {
                    let todo = res.json::<Todo>().await?;
                    let status = if todo.completed { "完了" } else { "未完了" };
                    println!("ID: {}", todo.id);
                    println!("タイトル: {}", todo.title);
                    println!("状態: {}", status);
                },
                StatusCode::NOT_FOUND => {
                    println!("ID: {}のTodoは見つかりませんでした", id);
                },
                _ => {
                    println!("エラー: {}", res.status());
                }
            }
        },
        
        Commands::Create { id, title } => {
            let new_todo = Todo {
                id,
                title,
                completed: false,
            };
            
            // JSONをプリントして確認
            println!("送信するJSON: {:?}", serde_json::to_string(&new_todo)?);
            
            let client = reqwest::Client::new();
            let res = client
                .post("http://localhost:3000/todos")
                .header("Content-Type", "application/json")  // ヘッダーを明示的に指定
                .json(&new_todo)
                .send()
                .await?;
            
            println!("リクエストURL: {}", res.url());
            let status = res.status();
            println!("ステータスコード: {}", status);
            
            if status.is_success() {
                let created_todo = res.json::<Todo>().await?;
                println!("新しいTodoを作成しました:");
                println!("ID: {}", created_todo.id);
                println!("タイトル: {}", created_todo.title);
            } else {
                // statusを先に取得しておいてから、text()を呼び出す
                let error_body = res.text().await?;
                println!("Todoの作成に失敗しました: {} - {}", status, error_body);
            }
        },
        
        Commands::Update { id, title, completed } => {
            let url = format!("http://localhost:3000/todos/{}", id);
            let updated_todo = Todo {
                id,
                title,
                completed,
            };
            
            let res = client
                .put(&url)
                .json(&updated_todo)
                .send()
                .await?;
            
            match res.status() {
                StatusCode::OK => {
                    println!("ID: {}のTodoを更新しました", id);
                },
                StatusCode::NOT_FOUND => {
                    println!("ID: {}のTodoは見つかりませんでした", id);
                },
                _ => {
                    println!("エラー: {}", res.status());
                }
            }
        },
        
        Commands::Delete { id } => {
            let url = format!("http://localhost:3000/todos/{}", id);
            let res = client.delete(&url).send().await?;
            
            match res.status() {
                StatusCode::OK => {
                    println!("ID: {}のTodoを削除しました", id);
                },
                StatusCode::NOT_FOUND => {
                    println!("ID: {}のTodoは見つかりませんでした", id);
                },
                _ => {
                    println!("エラー: {}", res.status());
                }
            }
        },
    }
    
    Ok(())
}