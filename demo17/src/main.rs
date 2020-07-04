// (1) define macro (learn to say hi)
macro_rules! hey {
 () => { println!("hi,nice to meet you!"); };
 ($xname:expr) => { // $xname参数名 expr限定符 表达式
    println!("Hello {}",$xname);
 };
 ($xname:expr,$yname:expr) => {
    println!("{} Hello {}",$xname,$yname);
 };
}

macro_rules! vecs {
    ( $($elem:expr),+ ) => { // 当传入多个是  需要括起来 加上匹配模式
        let mut sc:Vec<String> = Vec::new();
        $(
            sc.push(format!("{}",$elem));
        )+ // 重复操作代码块
        sc
    };
}

// mecro_rules! 定义宏
// mecro_rules! marcro_name {
//     () => { 重载 匹配
//         code blocks;
//     };
//     (pt1,pt2) => {
//         code blocks;
//     };
// }

fn main() {
    hey!();
    hey!("World");
    hey!("Dollar","Killer");
    println!("Hello, world!");

    let c = vecs![1,2,3,4,5,6];
    println!("c {:?}",c);
}

