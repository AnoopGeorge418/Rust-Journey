use std::collections::HashMap;

fn main() {
    println!("HashMap Collection!");

    // creating a empty HashMap
    let mut score = HashMap::new();

    // adding key-value pairs to the HashMap
    score.insert("Alice", 100);
    score.insert("Bob", 85);

    // accessing values from the HashMap
    let alice_score = score.get("Alice");
    println!("Alice's score: {:?}", alice_score);

    // looping through the HashMap
    for (key, value) in &score {
        println!("{}: {}", key, value);
    }

    // updating a value in the HashMap
    score.insert("Alice", 90);
    println!("Alice's updated score: {:?}", score.get("Alice"));

    // checking if value exists in the HashMap
    score.entry("Alice").or_insert(0);
    println!("Alice's score: {:?}", score.get("Alice"));
    println!("Bob's score: {:?}", score.get("Bob"));

    // removing a value from the HashMap
    score.remove("Bob");
    println!("Bob's score: {:?}", score.get("Bob"));
}
