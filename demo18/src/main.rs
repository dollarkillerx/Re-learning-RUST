fn func_add_org(xs: &mut Vec<i32>, ys: &Vec<i32>) {
    for (x, y) in xs.iter_mut().zip(ys.iter())
    {
        *x += *y;
    }
}

macro_rules! op {
    ($func:ident,$bound:ident,$method:ident) => {
        fn $func<T: $bound<T,Output=T> + Copy>(xs: &mut Vec<T>,ys:&Vec<T>) {
            for (x,y) in xs.iter_mut().zip(ys.iter())
               {
                *x = $bound::$method(*x,*y);
               }
        }
    };
}

op!(fund_add_new,Add,add);
op!(fund_sub_new,Sub,sub);
op!(fund_mul_new,Mul,mul);

fn main() {
    println!("Hello, world!");
    let mut x1 = vec![1, 2, 3];
    let mut x2 = vec![1, 2, 3];
    // func_add_org(&mut x1,&x2);
    fund_sub_new(&mut x1, &x2);
    println!("x1 : {:?}", x1);
}
