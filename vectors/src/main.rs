#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("Vector Collection!");

    // new empty vector collection
    let mut v: Vec<i32> = Vec::new();

    // vector macro - no type annotation required - auto inferred by compiler
    let w = vec![1, 2, 3];

    for i in &w {
        println!("{}", i);
    }

    // new line space
    println!("");

    // adding/pushing elements to vector using push method
    v.push(1);
    v.push(2);
    v.push(3);

    // iterating over vector
    for i in &v {
        println!("{}", i);
    }

    // accessing elements by index 
    let second: &i32 = &w[1];
    println!("Second Element is: {}", second);

    // accessing elements by get method 
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // new line space
    println!("");

    // adding different type of data in vector collection using enums.
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    for i in &row {
        match i {
            SpreadSheetCell::Int(i) => println!("Int: {}", i),
            SpreadSheetCell::Float(f) => println!("Float: {}", f),
            SpreadSheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}
