// global scope
const TIME_IN_SECONDS: u32 = 60 * 60 * 60;

fn main() { 
    println!("let Variable ---------------------------");
    // let x = 5; // immutable
    let mut x = 5; // mutable
    println!("The value of x is: {x}");
    x = 6; // assigning
    println!("The value of x is: {x}");

    // constant
    println!("Constant ---------------------------");
    const TIME: u32 = 60 * 60 * 3;
    println!("The time is: {TIME}");
    println!("Global Scope constant is: {TIME_IN_SECONDS}");

    // shadowing
    println!("Shadowing ---------------------------");
    // outer scope
    let x = 5;
    let x = x + 1;
    {
        // inner score
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    println!("Scalar Data Type ---------------------------");

    // floating point
    let x = 3.0; // f64
    let y: f32 = 3.0; // f32

    println!("The value of floating points x and y are: {x}, {y}");

    println!("Characters ---------------------------");
    let c = 'z';
    let z: char = 'Z';
    let emoji = '😻';
    println!("{c}, {z}, {emoji}");

    println!("Compound Data Type ---------------------------");
    let tup = (500, 6.4, 1);
    println!("The tuples are: {tup:?}");
    let (x, y, z) = tup; // destructuring
    println!("The value of x, y and z is: {x}, {y}, {z}");

    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [5; 10];
    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");

    println!("Accessing array elements");
    let first = a[0];
    println!("{first}");

    println!("Operations ---------------------------");
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!(
        "
        Addition: {sum}, 
        Subtraction: {difference}, 
        Multiplication: {product}, 
        Division: {truncated}. 
        Division: {quotient}
        Reminder: {remainder}, 
    "
    );
}
