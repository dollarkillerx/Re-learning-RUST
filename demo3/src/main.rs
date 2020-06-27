use std::io::stdin;
fn main() {
    println!("input:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("read stdin error");

    println!("You input: {1}, hello: {0}","1212",input.trim());
}
