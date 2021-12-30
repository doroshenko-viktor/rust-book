enum IpAddrKind {
    V4,
    V6,
}

pub fn simple_enum_usage_example() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // it is possible to create a function which will receive any ip address
    fn route(ip_kind: IpAddrKind) {}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn using_enum_as_struct_field() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

// but there is more concise way of doing this
enum IpAddrAssociatedWithType {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn example_associated_enum() {
    let home = IpAddrAssociatedWithType::V4(127, 0, 0, 1);
    let loopback = IpAddrAssociatedWithType::V6(String::from("::1"));
}

// This enum has four variants with different types:
// Quit has no data associated with it at all.
// Move has named fields like a struct does.
// Write includes a single String.
// ChangeColor includes three i32 values.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can define methods for this enum
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn example_enum_method() {
    let m = Message::Write(String::from("hello"));
    m.call()
}


