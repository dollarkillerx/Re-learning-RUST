use tokio::net::{TcpStream};
use tokio::io::{
    self,
    AsyncReadExt,
    AsyncWriteExt,
};

#[tokio::main]
async fn main() -> io::Result<()>{
    ttc().await
    // Ok(())
}


async fn ttc() -> io::Result<()> {
    let mut listener = TcpStream::connect("0.0.0.0:8081").await?;

    let (mut rd,mut wt) = io::split(listener);

    loop {
        let (mut socket,addr) = listener.accept().await?;
        tokio::spawn(async move {
            println!("addr: {}",addr.to_string());
            fc(socket).await;
        });   
    }
}

async fn fc(socket: TcpStream) {

}