fn main() {
    let cir1 = Circle::new();
    cir1.print();

    let tc = Tea{
        component: "xxx".to_string(),
        container: "cpp".to_string()
    };
    tc.make_drink();
}

trait Default {
    fn new() -> Self;
    fn print(self: &Self); // fn print(&self)
}

struct Circle {
    radius: f32,
}
impl Default for Circle {
    fn new() -> Circle {
        Circle{
            radius: 1.0,
        }
    }
    fn print(self: &Self) {
        println!("circle wit radius");
    }
}

struct Tea {
    component: String,
    container: String,
}

// 特性被现有的特性所约束
trait Routine {
    fn add_component(&self);
    fn add_container(&self);
}
trait Make: Routine {
    fn make_drink(&self) {
        self.add_component();
        self.add_container();
    }
}

impl Routine for Tea {
    fn add_component(&self) {
        println!("add component: {}",self.component);
    }

    fn add_container(&self) {
        println!("add component: {}",self.container);
    }
}
impl  Make for Tea{}
