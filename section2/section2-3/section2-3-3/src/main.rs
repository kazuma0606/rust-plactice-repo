use tokio::{sync::mpsc, time::sleep};
use std::time::Duration;

#[tokio::main]
async fn main() {
    // 通知チャンネル（送信側と受信側）
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // バックグラウンドで通知ワーカーを起動
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("🔔 通知送信中: {msg}");
            sleep(Duration::from_secs(2)).await;
            println!("✅ 通知送信完了: {msg}");
        }
    });

    // メイン処理：ユーザー操作など
    println!("📨 通知リクエスト受信: 'Rust学習が完了しました'");
    tx.send("Rust学習が完了しました".into()).await.unwrap();

    println!("👍 メイン処理完了（すぐにレスポンスを返せる）");

    // ワーカーが動作するのを確認するために少し待機
    sleep(Duration::from_secs(3)).await;
}
