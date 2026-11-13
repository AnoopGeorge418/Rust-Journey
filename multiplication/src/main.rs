use std::io::stdin;

fn main() {
    println!("Multiplication table!");

    let mut input = String::new();

    println!("Enter table number: ");
    stdin()
        .read_line(&mut input)
        .expect("Failed to read user input!");

    let number: i32 = input.trim().parse().expect("Enter a valid number");

    multiplication(number);
}

fn multiplication(number: i32) {
    println!("\nMultiplication Table of {number}\n");

    for i in 1..=10 {
        let res = number * i;
        println!("{number} * {i} = {res}");
    }
}
