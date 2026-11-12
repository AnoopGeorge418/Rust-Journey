fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    test_expressions();
    let x = five();
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("\t Hello from another function!"); // tab space
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("\n"); // empty new line
    println!("The measurement is: {value}{unit_label}");
}

fn test_expressions() {
    println!("\n");
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is {y}");
}

// fn with return 
fn five() -> i32 {
    println!("\n");
    5
}
