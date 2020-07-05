use std::thread;
use std::sync::{
    mpsc,
    Arc,   // 适合多线程的引用计数器 不可变
    Mutex, // 适合多线程的可变的
};

fn main() {
    println!("sync::mpsc rust线程间通信依赖mpsc  是多发送者单接收者   (实现多发送者多接收者需要Arc<Mutex>)");
    let (send,rec):(mpsc::Sender<i32>,mpsc::Receiver<i32>) = mpsc::channel();
    let rec_chanel = Arc::new(Mutex::new(rec));
    let mut tasks = Vec::new();

    for _i in 0..5 {
        let send1 = send.clone();
        let hand = thread::spawn(move|| {
            for i in 90..9999 {
            // for i in 0..10 {
                send1.send(i).unwrap();
            }
            println!("Send All Over");
        });
        tasks.push(hand);
    }
    drop(send);

    for _i in 0..10 {
        let rec = rec_chanel.clone();
        let hand = thread::spawn(move|| {
            loop {
                match rec.lock().unwrap().recv() {
                    Ok(data) => {
                        println!("recv data: {}",data);
                    },
                    Err(e) => { // 当无生产者 引用 send时 会收到err
                        println!("E: {:?}",e);
                        println!("OVER =      === == = = = = = = == = =");
                        break;
                    },
                }
            }

            // for i in rec.lock().unwrap().recv() {
            //     println!("read data: {}",i)
            // }
        });
        tasks.push(hand);
    }


    for i in tasks {
        i.join().unwrap();
    }

    println!("task over");
}
