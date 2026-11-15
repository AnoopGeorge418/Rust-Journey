#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // tuple type
    // let rect1 = (30, 50); 

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    
    println!("{rect1:?}");
    println!("{rect1:#?}");

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2); // another macro instead of println!{:?}
}

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
