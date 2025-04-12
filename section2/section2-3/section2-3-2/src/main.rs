use tokio::{
    join,
    time::{
        Duration, sleep
    },
};


#[tokio::main]
async fn main() {
    let task1 = async {
        println!("Task 1 開始");
        sleep(Duration::from_secs(2)).await;
        println!("Task 1 終了");
    };

    let task2 = async {
        println!("Task 2 開始");
        sleep(Duration::from_secs(3)).await;
        println!("Task 2 終了");
    };

    // 並行に実行
    join!(task1, task2);
    println!("すべてのタスクが完了しました");
}
