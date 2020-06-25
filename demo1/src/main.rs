
fn main() {
    // test1();
    // test2();
    // test3();
    // test4();
    test5();
}

// Scope
// Ownership Rule
// Copy amd Move
// Stacks and Heap
fn test2() {
    let mut s1 = String::from("str1");
    let s2 = &mut s1; // String默认没有实现Copy trait 所以赋值 发生了所有权转移
    // println!("s1: {}, sp: {:p}",&s1,&s1);
    println!("s2: {}, sp: {:p}",&s2,&s2);

    s2.push_str("hello fuck");

    // println!("s1: {}, sp: {:p}",&s1,&s1);
    println!("s2: {}, sp: {:p}",&s2,&s2);
    // 只能同时存在多个不可变借用  或者存在一个可变借用
}


// slice
fn test3() {
    let a = [1,2,3,4,5,6];
    let b = &a[..a.len()]; // 前闭后开 (请问这个为什么会用引用？  因为直接用值编译器在编译时无法知道数据大小)
    println!("b: {:?}",b);
    let str1 = String::from("Hello World");
    let p2 = &str1[..3];
    println!("p2: {:?}",p2);
}

fn test1() {
    let a = "hello".to_string();

    {
        let a = "ppc".to_string();
    }
    println!("a: {}",a);
    println!("Hello, world!");
}

#[derive(Debug)]
struct User<'a> { // struct使用引用做成成员变量必须标注生命周期
    age: &'a u8,
    name: &'a String,
}

fn test4() {
    let name = String::from("dollarkiller");
    let age:u8 = 17;

    let user = User{
        name:&name,
        age:&age,
    };
    println!("user: {:?}",user);
}

enum Pb {
    Ping(i8),
    Pang{
        msg: String,
        code: i16,
    },
}

// enum
fn test5() {
    let a = Pb::Pang{
        msg: "Pang".to_string(),
        code:12,
    };
    match &a {
        Pb::Pang{msg:m,code:_} => {
            println!("msg: {}",m);
        },
        Pb::Ping(x) => {
            println!("x: {}",x);
        }
    };

    if let Pb::Ping(x)= a {
        println!("x :{}",x);
    }else {
        println!("a not is Ping")
    }
}