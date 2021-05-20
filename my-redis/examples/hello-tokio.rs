use tokio::sync::mpsc;
use tokio::task;

async fn say_world() {
    println!("world");
}

fn sync_work() -> &'static str {
    "value"
}

async fn async_work() -> &'static str {
    "value"
}

#[tokio::main]
async fn main() {
    let handle = task::spawn(async { "task::spawn" });

    let op = say_world();
    println!("hello");
    op.await;

    let out = handle.await.unwrap();
    println!("return from {}", out);
    println!(
        "return from {}",
        tokio::spawn(async { "tokio::spawn" }).await.unwrap()
    );

    let handle = task::spawn(async { sync_work() });
    println!(
        "return from async_work() {}",
        task::spawn(async { async_work().await }).await.unwrap()
    );
    let out = handle.await.unwrap();
    println!("return from sync_work() {}", out);

    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    tokio::spawn(async move {
        tx.send("1").await.unwrap();
    });
    tokio::spawn(async move {
        tx2.send("2").await.unwrap();
    });
    while let Some(message) = rx.recv().await {
        println!("RECV {}", message);
    }
}
