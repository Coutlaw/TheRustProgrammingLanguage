// Define an enum
// ip address can be v4 or v6
enum IpAddrKind {
    V4,
    V6,
}

// function that can take any of the enumeration type
fn route(ip_kind: IpAddrKind) {}

// struct that uses enum as a prop
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(String),
    V6(String),
}

enum IpAddrEnumMix {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

// Custom enum where each type is a struct
enum IpAddrCustonStruct {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// error message enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


fn main() {
    // assign vars to an enum type
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // call functions with any of the enum types
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // enum but you can assign values to the types
    let home = IpAddrEnum::V4(String::from("127.0.0.1"));

    let loopback = IpAddrEnum::V6(String::from("::1"));

    // enum but you can mix and match types with primatives
    let home = IpAddrEnumMix::V4(127, 0, 0, 1);

    let loopback = IpAddrEnumMix::V6(String::from("::1"));


    
}
