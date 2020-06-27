// 首先 FnOnce/FnMut/Fn 这三个东西被称为 Trait,
// 默认情况下它们是交给rust编译器去推理的, 大致的推理原则是:
//     FnOnce: 当指定这个Trait时, 匿名函数内访问的外部变量必须拥有所有权.
//     FnMut: 当指定这个Trait时, 匿名函数可以改变外部变量的值.
//     Fn: 当指定这个Trait时, 匿名函数只能读取(borrow value immutably)变量值.

fn main() {
    let cmp = |x: u32,y:u32| -> bool {
        x > y
    };
    let x = 12;
    let y = 23;
    println!("x: {} y: {} cmp: {}",x,y,cmp(x,y));

    let cmp:fn(u32,u32) -> bool = |x: u32,y:u32| x > y;
    println!("x: {} y: {} cmp: {}",x,y,cmp(x,y));

    println!("x: {} y: {} cmp: {}",x,y,get_max(x,y,cmp));
    println!("x: {} y: {} cmp: {}",x,y,get_max_2(x,y,cmp));

    println!("Hello, world!");
}

fn get_max(x: u32,y: u32,opt: fn(u32,u32) -> bool) -> bool {
    opt(x,y)
}

fn get_max_2<T: Fn(u32,u32)->bool>(x:u32,y:u32,opt:T) -> bool {
    opt(x,y)
}