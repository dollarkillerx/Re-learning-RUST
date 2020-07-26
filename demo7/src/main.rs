use tokio::fs::File;
use tokio::io::{
    self,
    AsyncReadExt,
    AsyncWriteExt,
};

#[tokio::main]
async fn main() ->io::Result<()> {
    read_test1().await?;
    read_all().await?;
    cre_write().await?;
    write_all().await
    // crx().await
    // Ok(())
}

async fn read_test1() -> io::Result<()> {
    let mut f = File::open("src/main.rs").await?;  // you need AsyncReadExt
    let mut buffer = [0;10];

    let n = f.read(&mut buffer[..]).await?;
    // println!("thi bytes: {:?}",String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}

async fn read_all() -> io::Result<()> {
    let mut f = File::open("src/main.rs").await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;
    println!("{:?}",String::from_utf8(buffer));
    Ok(())
}

async fn cre_write() -> io::Result<()> {
    // create 不存在就创建 存在就返回句柄
    let mut file = File::open("src/foo.txt").await?;
    let n = file.write(b"ffc ").await?;  // you need AsyncWriteExt
    file.write(b"ffc ").await?;  // you need AsyncWriteExt
    file.write(b"ffc ").await?;  // you need AsyncWriteExt
    file.write(b"ffc ").await?;  // you need AsyncWriteExt
    file.flush(); // clear 
    // println!("write byte : {}",n);
    Ok(())
}

async fn write_all() -> io::Result<()> {
    let mut file = File::create("src/cpx.txt").await?;

    file.write_all(b"some bytes").await?;
    Ok(())
}

async fn crx() -> io::Result<()> {
    let mut cx = File::open("src/foo.txt").await?;
    let mut file = File::create("src/fo1.txt").await?;

    io::copy(&mut cx,&mut file).await?;
    Ok(())
}