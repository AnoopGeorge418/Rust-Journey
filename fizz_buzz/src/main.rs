fn main() {
    println!("Fizz Buzz!");

    fizz_buzz();
}

fn fizz_buzz() {
    for i in 1..=100 {
        if i % 15 == 0 {
            println!("Fizz Buzz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{i}");
        }
    }
}
