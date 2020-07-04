fn main() {
    test1();
    test2();
}

fn test1() {
    // Box
    let b1 = Box::new(1);
    let b2 = &b1;
    println!("box1: {}  box2: {}",b1,b2);

    // Box -> ptr 转为原始指针
    let box2 = Box::new(5); // 会发生转移
    let ptr2 = Box::into_raw(box2);
    unsafe {
        *ptr2 = 12;
    }
    println!("ptr2: {:?}",ptr2);
    let box3_bak = unsafe {Box::from_raw(ptr2)}; // 原始指针转换为Box

    // box leak 内存泄漏给引用
    let box3 = Box::new(3);
    let ref3 = Box::leak(box3);
    println!("ref3: {}",ref3);
}

// LIST
#[derive(Debug)]
struct Node<T> {
    _val: T,
    _ptr: Option<Box<Node<T>>>,
}
impl <T> Node<T> {
    fn new(item:T) -> Node<T> {
        Node{
            _val:item,
            _ptr:None,
        }
    }
}
#[derive(Debug)]
struct List<T> {
    _size: usize,
    _head: Option<Box<Node<T>>>,
}
impl <T> List<T> {
    fn new() -> List<T> {
        List{
            _size:0,
            _head: None,
        }
    }
    fn push(&mut self,item: T) {
        let new = Node::new(item);
        if self._size == 0 {
            self._head = Some(Box::new(new));
        }else {
            let mut node = &mut self._head;
            loop {
                match node {
                    Some(ok) => {
                        node = &mut ok._ptr;
                        continue;
                    },
                    None => {
                        *node = Some(Box::new(new));
                        break;
                    }
                };
            }
        }
        self._size += 1;
    }
    fn pop(&mut self) -> Option<T> {
        // let node = &mut self._head;
        match &self._head {
            Some(msg) => {
                // *node = msg._ptr;
                Some(msg._val)
            },
            None => None,
        }
    }
}
// impl <T> Iterator for List<T> {
//     type Item = T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let node = &mut self._head;
//         match node {
//             None => None,
//             Some(msg) => {
//                 *node = msg._ptr;
//                 Some(msg._val)
//             },
//         }
//     }
// }

fn test2() {
    let mut list = List::new();
    list.push("hello");
    list.push("hello1");
    list.push("hello2");
    list.push("hello3");
    list.push("hello4");
    list.push("hello5");

    // list.push(1);
    // list.push(2);
    // list.push(3);
    // list.push(4);
    // list.push(5);

    // for i in list.into_iter() {
    //     println!("i: {:?}",i);
    // }

    for _i in 0..5 {
        println!("data: {}",list.pop().unwrap());
    }
}