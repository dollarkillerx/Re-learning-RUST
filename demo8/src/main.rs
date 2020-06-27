fn main() {
    println!("Hello, world!");
    let s1 = String::from("hello");
    let s2 = String::from("world");
    println!("ppc: {}",long(&s1,&s2));

    println!("ccx: {}",get_version());

    let sc = User{
        name: &s1,
    };
    println!("sc: {}",sc.name);
}

fn long<'t1>(s1: &'t1 String,s2: &'t1 String) -> &'t1 String {
    if s1.len() > s2.len() {
        return s2
    }
    return s1
}

fn get_version() -> &'static str {
    "version 0.1"
}

struct User<'t1> {
    name: &'t1 str,
}
impl <'a> User <'a> {
    fn print(&self) {
        println!("name: {}",self.name);
    }
}