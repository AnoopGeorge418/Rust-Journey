use std::io::stdin;

fn main() {
    println!("Prime Number Checker!");

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let number: i32 = input.trim().parse().expect("Please enter a valid number.");

    let res: &str = prime_number_checker(number);
    println!("The Number {number} is a: {res}");
}

fn prime_number_checker(num: i32) -> &'static str {
    if num <= 1 {
        return "not prime number";
    }

    for i in 2..num {
        if num % i == 0 {
            return "Not a prime number";
        }
    }

    "Prime number"
}
