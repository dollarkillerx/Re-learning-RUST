use futures::executor::block_on;

async fn hello_world() {
    println!("Hello World");
}

fn test_base() {
    let hello = hello_world();
    block_on(hello);
}

type Song = fn();

async fn learn_and_song() {
    let sg:Song = learn_song().await;
    song(sg).await;
}

async fn learn_song() -> Song {
    println!("learn song");
    ||{
        println!("小星星")
    }
}

async fn song(song: Song) {
    song();
}

async fn dancing() {
    println!("dancing");
}

async fn performance() {
    let f1 = learn_and_song();
    let f2 = dancing();

    futures::join!(f1,f2);
}

fn main() {
    block_on(performance());
    // 这里的测试结果  貌似还是同步代码
}
