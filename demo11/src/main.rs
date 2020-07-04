use std::ops::{Deref, DerefMut};

fn main() {
    let c = Packing::new(String::from("avc"));
    let b = &c;
    println!("c : {:p}",&c);
    println!("b : {:p}",b);

    let s1 = Packing::new(String::from("avc"));
    let mut s2 = Packing::new(String::from("avc"));

    println!("s1: len {}",s1.len());


}

#[derive(Debug)]
struct Packing<T> {
    obj: T,
}
impl <T> Packing<T> {
    fn new(x : T) -> Packing<T> {
        Packing{
            obj:x,
        }
    }
}

impl <T> Deref for Packing<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}

impl <T> DerefMut for Packing<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj
    }
}

