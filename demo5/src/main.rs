trait Humanity {
    fn sing(&self);

    fn jump(&self) {
        println!("jump");
    }

    fn rap(&self);
    fn keyboard_man(&self) {
        println!("all keyboard man");
    }
}

struct DollarKiller {}

impl Humanity for DollarKiller {
    fn sing(&self) {
        println!("sing");
    }

    fn rap(&self) {
        println!("rap");
    }

    fn keyboard_man(&self) {
        println!("no keyboard man");
    }
}

fn main() {
    let dollar_killer = DollarKiller{};
    dollar_killer.rap();


    println!("Hello, world!");
}
