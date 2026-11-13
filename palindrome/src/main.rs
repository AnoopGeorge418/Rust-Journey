use std::io::stdin;

fn main() {
    println!("Fibonacci Sequence!");

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let number: i32 = input.trim().parse().expect("Please enter a valid number.");

    palindrome_checker(number);
}

fn palindrome_checker(number: i32) {
    let mut temp = number;
    let mut reversed = 0;

    while temp > 0 {
        let digit = temp % 10;
        reversed = reversed * 10 + digit;
        temp /= 10;
    }
    
    if reversed == number {
        println!("Number {number} is: Palindrome");
    } else {
        println!("Number {number} is: Not a Palindrome");
    }
}
