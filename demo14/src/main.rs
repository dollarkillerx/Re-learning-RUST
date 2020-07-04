use std::ops::Add;

fn main() {
    println!("Hello, world!");

    let a = AType{ss:String::from("acp")};
    let b = AType{ss:String::from("apc")};
    println!("{}",a + b);
}

struct AType {
    ss: String,
}
impl Add for AType {
    type Output = String;

    fn add(self, rhs: Self) -> Self::Output {
        let mut s = self.ss;
        s.push_str(&rhs.ss);
        s
    }
}
