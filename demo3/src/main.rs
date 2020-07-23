use mini_redis::{client,Frame, Result, Connection};
use tokio::net::{TcpListener,TcpStream};

#[tokio::main]
pub async fn main() ->Result<()> {
    // test1().await;
    // test2().await
    test3().await;

    Ok(())
}

async fn test3() {
    // Bind the listener to the address
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the ip and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
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
