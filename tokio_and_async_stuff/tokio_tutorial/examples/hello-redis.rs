use mini_redis::{client,Result};

#[tokio::main]
async fn main() -> Result<()> {
    // e open a connection to a mini redis address
    let mut client = client::connect("127.0.0.1:6379").await?;

    // e make a key value pair : "hello" -> "world"
    client.set("hello","world".into()).await?;

    // e do a get call, using "hello" as key
    let result = client.get("hello").await?;

    println!("Got value from server; result : {:?}", result);

    Ok(())
}