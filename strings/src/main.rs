fn main() {
    println!("Strings Collection!");

    // creating empty new string 
    let mut _s = String::new();
    
    // storing initial value 
    let data = "some data content".to_string();

    // creating string from literal 
    let mut s1 = String::from("some initial contents ");

    // appending to a string using push_str
    s1.push_str("and some appended content");
    println!("s1 with push_str: {}", s1);

    // appending to a string using push 
    s1.push('l');
    println!("s1 with push: {}", s1);

    // combining/concatenating strings using + operator
    let s3 = s1 + &data;
    println!("s3 concatenated data: {}", s3);

    // concatenating strings using format! macro
    let s4 = format!("{s3} - hello world");
    println!(" s4 concatenated data: {}", s4);

    // utf encoded strings
    let hello = String::from("السلام عليكم");
    println!("hello: {}", hello);
    let hello = String::from("Dobrý den");
    println!("hello: {}", hello);
    let hello = String::from("Hello");
    println!("hello: {}", hello);
    let hello = String::from("שלום");
    println!("hello: {}", hello);
    let hello = String::from("नमस्ते");
    println!("hello: {}", hello);
    let hello = String::from("こんにちは");
    println!("hello: {}", hello);
    let hello = String::from("안녕하세요");
    println!("hello: {}", hello);
    let hello = String::from("你好");
    println!("hello: {}", hello);
    let hello = String::from("Olá");
    println!("hello: {}", hello);
    let hello = String::from("Здравствуйте");
    println!("hello: {}", hello);
    let hello = String::from("Hola");
    println!("hello: {}", hello);
}
