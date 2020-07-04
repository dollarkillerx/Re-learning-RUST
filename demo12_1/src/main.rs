// struct Handbag<T> {
//     article: Option<Box<T>>,
// }
// impl <T>Handbag<T> {
//     fn new() -> Handbag<T> {
//         Handbag{
//             article:None,
//         }
//     }
//     fn put(&mut self,article: T) {
//         self.article = Some(Box::new(article));
//     }
//     fn take(&mut self) -> Option<Box<T>> {
//         match &self.article {
//             Some(data) => { // data: &Box<T>
//                 self.article = None;
//                 return Some(*data)
//             },
//             None => None,
//         }
//     }
// }

use std::mem::take;

fn main() {
    // let mut handbag = Handbag::new();
    // handbag.put(String::from("hello"));
    // if let Some(article) = handbag.take() {
    //     println!("data: {}",article);
    // }

    let mut u = User::new();
    u.set_name(String::from("acg"));
    println!("name: {}",u.get_name().unwrap());
    println!("name: {}",u.get_name().unwrap());

    println!("Hello, world!");
}

struct User {
    name: Option<String>,
}
impl User {
    fn new() -> User {
        User{name:None}
    }
    fn set_name(&mut self,name: String) {
        self.name = Some(name);
    }
    fn get_name(&mut self) -> Option<String> {
        take(&mut self.name)
        // match &self.name {
        //     None=>None,
        //     Some(name) => {
        //         take(name)
        //     },
        // }
    }
}