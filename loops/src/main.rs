fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    nested_loop();
    condition_with_while();
    looping_collections();
    using_range();
    using_range_reverse();
}

fn nested_loop() {
    println!("Nested loop");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn condition_with_while() {
    println!("While loop");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFT OFF!!!");
}

fn looping_collections() {
    println!("For loop");
    let a = [10, 20, 30, 40, 50, 60];

    for element in a {
        println!("The value is: {element}: ");
    }
}

fn using_range() {
    for number in 1..5 {
        println!("{number}");
    }
    println!("LIFT OFF!!!")
}

fn using_range_reverse() {
    for number in (1..5).rev() {
        println!("{number}");
    }
    println!("LIFT OFF!!!")
}

