fn main() {
    // println!("Hello, world!");
    // enums give you a way of saying a value is one of a possible set of values.

    enum IpAddrKind {
        V4,
        V6,
    }
    fn route(ip_kind: IpAddrKind) {}
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    //moreover, we can make us structs to store values of them as well
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //but we can do better using enums:
    enum IpAddrE {
        V4(String),
        V6(String),
    }

    let home = IpAddrE::V4(String::from("127.0.0.1"));

    let loopback = IpAddrE::V6(String::from("::1"));

    //all variants can have different types
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
}
