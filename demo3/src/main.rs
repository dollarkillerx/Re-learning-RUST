use mini_redis::{client,Frame, Result, Connection};
use tokio::net::{TcpListener,TcpStream};
use std::collections::HashMap;
use mini_redis::Command::{self,Get,Set};


#[tokio::main]
pub async fn main() ->Result<()> {
    // test1().await;
    // test2().await
    test3().await;

    Ok(())
}

async fn test3() {
    // Bind the listener to the address
    let mut listener = TcpListener::bind("127.0.0.1:6380").await.unwrap();

    loop {
        // The second item contains the ip and port of the new connection.
        let (socket, socket_addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            println!("in addr: {}",socket_addr.to_string());
            // process(socket).await;
            process_pro(socket).await;
        });
    }
}

async fn process_pro(socket: TcpStream) {
    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);
    
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
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

    print!("Hell In Progress
    C2V + DOo ");

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
