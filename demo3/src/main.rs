use mini_redis::{client,Result};

#[tokio::main]
pub async fn main() ->Result<()> {
    test2().await
}

async fn test2() -> Result<()> {
    let op = say_world();

    print!("Hello ");

    op.await;
    Ok(())
}

async fn say_world() {
    println!("World");
}

async fn test1() -> Result<()> {
    // connection mini-redis
    let mut client = client::connect("127.0.0.1:8089").await?;

    // set
    client.set("hello","world".into()).await?;
    // get
    let val = client.get("hello").await?;

    println!("result: {:?}",val);

    Ok(())
}
