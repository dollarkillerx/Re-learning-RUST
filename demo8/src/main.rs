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

    // 读写分离
    let (mut rd,mut wr) = io::split(listener);


    // send
    let write_task = tokio::spawn(async move {
        wr.write_all(b"Hello\r\n").await?;
        wr.write_all(b"World\r\n").await?;

        Ok::<_,io::Error>(())
    });

    let mut buf = vec![0;128];

    loop {
            let n = rd.read(&mut buf).await?;
            if n == 0 { // n == 0 read end
                break;
            }

            println!("GOT: {:?}",&buf[..n]);
    }

    Ok(())
}

async fn fc(socket: TcpStream) {
    
}