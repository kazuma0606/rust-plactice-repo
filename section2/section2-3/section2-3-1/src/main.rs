use tokio::time::{
    sleep,
    Duration,
};

#[tokio::main]
async fn main() {
    println!("処理A: 開始");
    async_task().await;
    println!("処理B: 終了");
}

async fn async_task() {
    println!("-> 非同期処理中...");
    // 2秒待機
    // ここでawaitを使うことで、非同期処理が完了するまで待機します。
    sleep(Duration::from_secs(2)).await;
    println!("-> 非同期処理完了!");
}