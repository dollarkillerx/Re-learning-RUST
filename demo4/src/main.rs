use tokio::task;
use tokio::task::yield_now;
use std::rc::Rc;


#[tokio::main]
async fn main() {
    // test1().await;
    // test2().await;
    // test2_p().await;
}



async fn test2() {
    tokio::spawn(async {
        {
            let rc = Rc::new("Hello World");
            println!("{}",rc);
        }

        test1().await;
    });
}

async fn test2_p() {   // tokio::spawn 必须是Send 
    tokio::spawn(async {

        yield_now().await;
        {
            let rc = Rc::new("Hello World");   // RC是不满足的  需要包起来
            println!("{}",rc);
        }
    });
}

async fn test1() {
    let handle = tokio::spawn(async {
        println!("加啊就啊");
        "hell worl"
    });

    tokio::spawn(async{
        println!("Hack")     
    });

    println!("s1");
    let out = handle.await.unwrap();
    println!("GOT {}",out);

    // s1
    // 加啊就啊
    // GOT hell worl
    // Hack
}