use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client1 = client::connect("127.0.0.1:6379").await?;
    let mut client2 = client::connect("127.0.0.1:6379").await?;

    client1.set("hello", "world".into()).await?;

    let result = client2.get("hello").await?;
    println!("got value from the server; result={:?}", result);
    let result = client1.get("hello").await?;
    println!("got value from the server; result={:?}", result);

    Ok(())
}
