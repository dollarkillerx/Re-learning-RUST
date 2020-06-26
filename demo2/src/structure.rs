use std::error::Error;
use std::thread::sleep;

pub struct LinkQueue<T> {
    next: Option<Box<LinkQueueItem<T>>>,
    len: usize,
}

struct LinkQueueItem<T> {
    data: T,
    next: Option<Box<LinkQueueItem<T>>>,
}

impl <T> LinkQueueItem<T> {
    fn new(data: T) -> LinkQueueItem<T> {
        LinkQueueItem {
            data: T,
            next: None,
        }
    }
}

impl<T> LinkQueue<T> {
    fn new() -> Self<T> {
        LinkQueue {
            len: 0,
            next:None,
        }
    }

    fn append(&mut self,data: T) -> Result<(),Box<dyn Error>>{
        let item = LinkQueueItem::new(data);
        let ne = &self.next;
        loop {
           match ne {
               Some(ns) => {
                   ne = Some(ns);
               },
               None => {
                    *ne.next = Box::new(item);
                    break;
               },
           }
        }
    }

}
