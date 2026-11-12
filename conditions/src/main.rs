fn main() {
    let number = 3;
    let condition = true;

    if number < 5 {
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }

    just_condition(number);
    combine_conditions(number);

    // condition in let
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

fn just_condition(number: u32) {
    if number != 0 {
        println!("number was something other than zero");
    }
}

fn combine_conditions(number: u32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
}
