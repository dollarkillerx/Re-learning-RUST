fn main() {
    test1(); // 迭代器
    test2(); // 迭代器
    test3(); // 自己实现迭代器
}

// 迭代器
fn test1() {
    let mut c:[u8;12] = [1,2,3,4,5,6,7,8,9,1,1,1];
    for i in c.iter() { // c.inter() 转为迭代器
        // println!("idx: {}, data: {}",i);
        println!("data: {}",i);
    }

    for i in c.iter_mut() { // 可变迭代器
        println!("data: {}",i);
    }

    for i in c.into_iter() { // 这里会发生转移 BUT这里数组是u8实现了Copy所以看不出效果
        println!("into iter : {}",i);
    }
    println!("c {:?}",c);

    let v2 = vec![1,2,3];
    for i in v2.iter() {
        println!("i: {}",i);
    }

    for v in v2.into_iter() {
        println!("C data : {}",v);
    }

    // println!("v2 vec: {:?}",v2); 上面into_iter时已经被借走了

    let v3 = vec![1,2,3,4,5,6,7,8,9,10];
    let v3_iter = v3.iter();
    let v3_pro: Vec<_> = v3_iter.collect(); // 迭代器转数组
    println!("v3 pro : {:?}",v3_pro);

    let v3_into = v3.into_iter(); // 生成一个拥有元素所有权的迭代器
    let v3_pp: Vec<i32> = v3_into.collect(); // 此处就返回完整所有权 而不是借用
    println!("v3 pp : {:?}",v3_pp);
}

// 迭代器
fn test2() {
    let mut v1 = vec![1,2,3];
    let v1_iter = v1.iter_mut();

    let f = | x:&mut u8 | *x * *x + 2;

    let v1_iter = v1_iter.map(f);
    let v2:Vec<_> = v1_iter.collect(); // Vec<_> _ 编译器自动推测类型
    println!("v2: {:?}",v2);

    // filter
    let v1: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10];
    let v1_iter = v1.iter();
    let f2 = |x: &&u8| -> bool {**x % 2 != 0};
    let v1_iter = v1_iter.filter(f2);
    let v1:Vec<_> = v1_iter.collect();
    println!("v1: {:?}",v1);

    // ZIP
    let v1 = vec![1,3,5];
    let v2 = vec![2,4,6];

    let v1_iter = v1.iter();
    let v2_iter = v2.iter();
    let v1_iter = v1_iter.zip(v2_iter);
    let v1: Vec<_> = v1_iter.collect();
    println!("v1 : {:?}",v1);// v1 : [(1, 2), (3, 4), (5, 6)]
    // vec + vec -> map(lens)
}

struct List<T> {
    core: Vec<T>,
}
impl <T> List<T> {
    fn new() -> List<T> {
        List{
            core: Vec::new(),
        }
    }
    fn push(&mut self,item: T) {
        self.core.push(item);
    }
}
impl <T>Iterator for List<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.core.len() == 0 {
            return None;
        }
        let c = self.core.pop();
        c
    }
}

fn test3() {
    println!("Test3");
    let mut c = vec![1,2,3,4,5,6];
    c = c[1..c.len()].to_owned();
    println!("c : {:?}",c);
    let b = c[0];
    println!("b : {}",b);
    let b = c[0];
    println!("b : {}",b);


    let mut c = List::new();
    c.push(121);
    c.push(1344);
    for v in c.into_iter() {
        println!("v :{}",v);
    }
}