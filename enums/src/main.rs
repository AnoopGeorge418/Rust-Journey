#[derive(Debug)]
enum IpVersions {
    V4, 
    V6, 
}

#[derive(Debug)]
struct IpAddress {
    version: IpVersions,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("Enumerations and Pattern Matching");

    let version_four = IpVersions::V4;
    let version_six = IpVersions::V6;

    let home = IpAddress {
        version: IpVersions::V4,
        address: String::from("127.0.0.1"),
    }; 
    let loopback = IpAddress {
        version: IpVersions::V6,
        address: String::from("::1"),
    };

    ip(version_six);
    ip(version_four);

    data(home, loopback);
}

fn ip(version: IpVersions) {
    println!("{version:?}");
}

fn data(home: IpAddress, loopback: IpAddress) {
    println!("Home: {home:?}, Loopback: {loopback:?}");
}
