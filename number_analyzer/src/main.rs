use std::io::stdin;

fn main() {
    println!("Number Analyzer!");

    // storing input from user
    let mut input = String::new();

    // getting input from user --> terminal
    stdin()
        .read_line(&mut input)
        .expect("Failed to read user input!");

    // converting string input to i32
    let number: i32 = input.trim().parse().expect("Enter a valid number");

    // call functions
    let number_state: String = check_sign(number);
    let odd_even: String = check_parity(number);
    let divisible_3: &str = is_divisible_by_3(number);
    let divisible_5: bool = is_divisible_by_5(number);
    let divisible_3_5: &str = is_divisible_by_both(number);

    println!("The number {number} is a: {number_state} number.");
    println!("The number {number} is a: {odd_even} number.");
    println!("The number {number} is: {divisible_3}.");
    println!("Divisible by 3: {divisible_5}.");
    println!("The number {number} is: {divisible_3_5}.");
}

fn check_sign(number: i32) -> String {
    if number > 0 {
        String::from("Positive")
    } else if number < 0 {
        String::from("Negative")
    } else {
        String::from("Zero")
    }
}

fn check_parity(number: i32) -> String {
    if number % 2 == 0 {
        "Even".to_string()
    } else {
        "Odd".to_string()
    }
}

fn is_divisible_by_3(number: i32) -> &'static str {
    if number % 3 == 0 {
        "Divisibile by 3"
    } else {
        "Not divisible by 3"
    }
}

fn is_divisible_by_5(number: i32) -> bool {
    number % 5 == 0
}

fn is_divisible_by_both(number: i32) -> &'static str {
    if (number % 3 == 0) && (number % 5 == 0) {
        "Divisibile by both 3 and 5"
    } else {
        "Not divisible by 3 and 5"
    }
}
