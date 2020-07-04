fn main() {
    println!("Copy Move Clone!");
    let wang = User{name:String::from("Dollar")}; // 不可变
    let wang3 = wang.clone(); // 复制一块内存
    let mut wang2 = wang; // 移动 并赋予可变性
    wang2.name.push_str("Killer");
    println!("name: {}",wang2.name);
    wang2.clone_from(&wang3);
    println!("name: {}",wang2.name);
}

#[derive(Clone)]
struct User {
    name: String,
}