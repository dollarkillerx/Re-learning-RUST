use demo2::*;
fn main() {
    let mut link_queue = structure::LinkQueue::new();
    link_queue.append("hello");
    link_queue.append("hello2");
    link_queue.append("hello3");

    for i in 0..link_queue.len() {
        println!("idx: {} val: {}",i,link_queue.next());
    }
    println!("Hello, world!");
}
