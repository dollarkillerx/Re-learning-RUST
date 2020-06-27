use std::error::Error;

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
            data,
            next: None,
        }
    }
}

impl<T> LinkQueue<T> {
    pub fn new() -> Self {
        LinkQueue {
            len: 0,
            next:None,
        }
    }

    pub fn append(&mut self,data: T) -> Result<(),Box<dyn Error>>{
        self.len += 1;
        let item = LinkQueueItem::new(data);
        let mut ne = &self;
        loop {
           match &ne.next {
               Some(ns) => {
                   ne.next = ns.next;
               },
               None => {
                    ne.next = Some(Box::new(item));
                    return Ok(())
               },
           }

        }
    }

    pub fn next(&mut self) -> Option<T> {
        match &self.next {
            None => None,
            Some(item) => {
                self.len -= 1;
                self.next = item.next;
                Some(item.data)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
