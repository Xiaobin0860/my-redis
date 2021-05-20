use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    //send on a oneshot channel will always fail or succeed immediately
                    let _ = resp.send(res);
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        }
    });

    let tx2 = tx.clone();
    let tx3 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };
        if tx.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }
        let res = resp_rx.await.unwrap();
        println!("GET return {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };
        if tx2.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }
        let res = resp_rx.await.unwrap();
        println!("SET return {:?}", res);
    });

    let t3 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };
        if tx3.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }
        let res = resp_rx.await.unwrap();
        println!("GET return {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    t3.await.unwrap();
    manager.await.unwrap();
}
