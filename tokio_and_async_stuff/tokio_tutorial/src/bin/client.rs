use mini_redis::client;
use tokio::sync::{mpsc,oneshot};
use mini_redis::Command::{Get,Set};
use bytes::Bytes;

#[derive(Debug)]
enum Command{
    Get{
        key: String,
        resp:Responder<Option<Bytes>>,
    },
    Set{
        key:String,
        val:Bytes,
        resp:Responder<()>,
    }
}

use oneshot::Sender;
use mini_redis::Result;

type Responder<T> = Sender<Result<T>>; // e means : Sender -> represents sender(tx) part of channel , Result<T> -> data type that'll be sent in the channel by this sender

#[tokio::main]
async fn main(){
    let (tx,mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move{

        let (tx_one, rx_one) = oneshot::channel();

        let cmd = Command::Get{
            key: "foo".to_string(),
            resp: tx_one,
        };

        tx.send(cmd).await.unwrap();

        let res = rx_one.await;
        println!("Got : {:?}", res);
    });

    let t2 = tokio::spawn(async move{

        let (tx_one, rx_one) = oneshot::channel();

        let cmd = Command::Set{
            key: "foo".to_string(),
            val: "bar".into(), // e convert to bytes
            resp: tx_one,
        };

        tx2.send(cmd).await.unwrap();

        let res = rx_one.await;
        println!("Got : {:?}", res);
    });

    let manager = tokio::spawn(async move{ // e `move` is for `rx`
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            // cmd is a enum - Command::Get, Command::Set
            use Command::*;

            match cmd {
                Get {key, resp} => {
                    let res = client.get(&key).await;
                    // e ignore the errors in the following line:
                    let _ = resp.send(res); // e no need for await, sending(or erroring-out is instant)
                }
                Set {key, val, resp} => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        };
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();

}