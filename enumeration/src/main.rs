struct Ipv4Addr {
    value: [u8; 4],
}

struct Ipv6Addr {
    value: String,
}

enum IpAddrKind {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message::call");
    }
}

fn main() {
    let home = IpAddrKind::V4(Ipv4Addr { value: [127, 0, 0, 1] });
    let loopback  = IpAddrKind::V6(Ipv6Addr { value: String::from("::1") });

    route(home);
    route(loopback);
}

fn route(_ip_type: IpAddrKind) {
}
