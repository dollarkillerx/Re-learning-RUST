fn main() {
    test1();
    test2();
}

fn test1() {
    println("Hello, world!");
    println(String::from("Fuck You!!!"));

    println2("xxx2  ");
    print_max(1,2);
    print_max(2,1);
}

fn println<T:std::fmt::Display>(x: T) {
// fn println<T>(x: T)   {  如果要调用泛型的方法 需要限定
    println!("x: {}",x);
}

fn println2<T>(x: T)
where T: std::fmt::Display  // 约定分离
{
    println(x);
}

fn print_max<T>(x: T,y: T)
where T: std::fmt::Display + std::cmp::PartialOrd
{
    if x < y {
        println(x);
    }else {
        println(y);
    }
}

/////////

fn test2() {
    let poi = Point{
        x: 12,
        y: 25,
    };
    println(poi.distance_from_origin());
    let poi2 = Point {
        x:123,
        y: 34,
    };

    println(Point::distance_2(&poi,&poi2));

    let c = Point::<f32>{
        x: 1.2,
        y: 1.4,
    };
    println(c.distance_from_origin_f32());
}

struct Point<T> {
    x: T,
    y: T,
}
impl <T> Point<T>
where T: Copy + std::ops::Add<Output=T> + std::ops::Mul<Output=T>
{
    fn distance_from_origin(&self) -> T {
        // COPY
        // ADD AMD MUL
        // return type
        return self.x + self.y
    }

    fn distance_2(p1: &Point<T>,p2: &Point<T>) -> T {
        (p1.x + p2.x) * (p2.y + p1.y)
    }

    fn show<U>(p: &Point<U>)
    where U: std::fmt::Display
    {
        println!("show point: ({},{})",p.x,p.y);
    }
}

impl Point<f32> {
    fn distance_from_origin_f32(&self) -> f32 {
        // COPY
        // ADD AMD MUL
        // return type
        return self.x + self.y + self.x * self.y;
    }
}