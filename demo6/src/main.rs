use tokio::sync::oneshot;
use tokio::time::delay_for;
use std::time::Duration;

#[tokio::main]
async fn main() {
    test_oneshot().await;
    loop {
        delay_for(Duration::from_secs(5)).await;
    }
}

async fn test_oneshot() {
    let (send,rec) = oneshot::channel();

    tokio::spawn(async move {
        delay_for(Duration::from_secs(5)).await;
        println!("resp");
        let rsp = rec.await;
        println!("get: {:?}",rsp);
    });

    tokio::spawn(async move {
        let e = send.send("hello world"); // oneshot send 只能发一次
        println!("srs: {:?}",e);
    });
}
