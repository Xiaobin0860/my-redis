use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() -> io::Result<()> {
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

    println!("{:?}", std::env::current_dir()?);
    println!("{:?}", std::env::current_exe()?);
    let exe_dir = std::env::current_exe()?.parent().unwrap().to_owned();
    std::env::set_current_dir(exe_dir)?;
    println!("{:?}", std::env::current_dir()?);

    let mut file = File::create("foo.txt").await?;
    file.write_all(b"some bytes").await?;
    println!("Wrote 'some bytes'");

    let mut file = File::open("foo.txt").await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    println!("{:?}", buffer);

    Ok(())
}

async fn say_world() {
    println!("world");
}

fn sync_work() -> &'static str {
    "value"
}

async fn async_work() -> &'static str {
    "value"
}
