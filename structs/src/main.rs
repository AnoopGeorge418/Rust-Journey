// creating a struct type
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Struct Learning!");

    // instance of struct
    let mut user_data = User {
        active: true,
        username: "anoop_george".to_string(),
        email: "anoopgeorge@gmail.com".to_string(),
        sign_in_count: 1,
    };

    // accessing a field from struct
    println!("User Email: {}", user_data.email);

    // changing the value of field
    user_data.email = "anoopgeorge418@gmail.com".to_string();
    println!("Updated User Email: {}", user_data.email);

    println!("The user data: {user_data:#?}");

    // struct update
    let user2 = User {
        email: String::from("demoshade70@gmail.com"),
        ..user_data
    };

    println!(
        "Active: {}, Username: {}, Sign-in count: {}",
        user2.active, user2.username, user2.sign_in_count
    );

    // usage - styled
    println!("The user data: {user2:#?}");

    tuple_struct();
}

fn tuple_struct() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black: {black:#?}, Origin: {origin:#?}");
}
