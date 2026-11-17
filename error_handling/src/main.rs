use std::fs::File;
use std::io::ErrorKind;

// how Result type works
// enum Result<T, E> {
//     Ok<T>,
//     Err<E>,
// }

fn main() {
    println!("Error handling!");

    // calling panic macro
    // panic!("crash and burn"); // any code after this is unreachable

    // Result type - error handling
    let greeting_file = File::open("hello.txt");
    let greetings = match greeting_file {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {error:?}"),
        // matching multiple variants of errors
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}")
            }
        },
    };
    println!("{greetings:?}");
}
