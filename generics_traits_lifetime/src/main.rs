struct Point<T> {
    x: T,
    y: T,
}

trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

fn main() {
    println!("Generic Types, Traits and Lifetime!");

    let point = Point { x: 5, y: 10 };
    println!("Point: ({}, {})", point.x, point.y);

    let dog = Dog;
    dog.make_sound();
    
}