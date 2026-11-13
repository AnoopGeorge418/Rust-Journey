use std::io::stdin;

fn main() {
    println!("Fibonacci Sequence!");

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let number: i32 = input.trim().parse().expect("Please enter a valid number.");

    let res: i32 = fibonacci(number);
    println!("Factorial of {number} is: {res}");
}

fn fibonacci(number: i32) -> i32 {
    let mut current_val: i32 = 0;
    let mut previous_sum: i32 = 1;
    let mut result: i32 = 0;

    for _ in 0..number {
        println!("{current_val}");

        result = current_val + previous_sum;
        current_val = previous_sum;
        previous_sum = result;
    }

    result
}
