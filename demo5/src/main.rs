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

struct Pass {}
impl Humanity for Pass {
    fn sing(&self) {
        println!("pass sing");
    }

    fn rap(&self) {
        println!("pass rap");
    }
}

fn sing(humanity: &impl Humanity) {
    humanity.sing();
}

fn get_humanity(sig: u8) -> Box<dyn Humanity> {
    return if sig > 8 {
        Box::new(DollarKiller {})
    } else {
        Box::new(Pass {})
    }
}

fn main() {
    let dollar_killer = DollarKiller{};
    dollar_killer.rap();

    sing(&dollar_killer);

    let dollar_killer = get_humanity(1);
    dollar_killer.keyboard_man();

    println!("Hello, world!");
}
