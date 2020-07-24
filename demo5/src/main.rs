use tokio::{
    net::{
        TcpListener,
        TcpStream,
    }
};
use std::{
    error::Error,
    collections::HashMap,
    sync::{Arc,Mutex},
};
use mini_redis::{Connection,Frame};
use mini_redis::Command::{self,Get,Set};
use bytes::Bytes;
// use mini_redis::Command;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:8989").await?;

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket,addr) = listener.accept().await?;
        
        let db = db.clone();
        tokio::spawn(async move {
            println!("IN addr: {}",addr.to_string());
            process(socket,db).await;        
        });
    }
}

async fn process(socket: TcpStream,db: Db) {

    let mut connection = Connection::new(socket);
    
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                }else {
                    Frame::Null
                }
            },
            cmd => panic!("unimplemented {:?}",cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}