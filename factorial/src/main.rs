use std::io::stdin;

fn main() {
    println!("Factorial of Number!");

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let number: i32 = input.trim().parse().expect("Please enter a valid number.");

    let res: i32 = factorial(number);
    println!("Factorial of {number} is: {res}");
}

fn factorial(num: i32) -> i32 {
    let mut i = 1;
    let mut res = 1;
    
    while i <= num {
        res *= i;
        i += 1;
    } 

    res
}
