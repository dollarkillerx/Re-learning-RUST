fn main() {
    println!("High Order Function !");
    let mut fname:IOE = inc_by_one;
    let x = 3;
    println!("opt {} = {}",x ,fname(x));
    fname = dec_by_one;
    println!("opt {} = {}",x ,fname(x));

}

// define HOF
type IOE = fn(i32) -> i32;
fn inc_by_one(x: i32) -> i32 {
    x + 1
}
fn dec_by_one(x: i32) -> i32 {
    x - 1
}