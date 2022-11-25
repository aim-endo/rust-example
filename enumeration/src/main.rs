enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback  = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);
}

fn route(_ip_type: IpAddrKind) {
    //println!("ip type {:?}", ip_type);
}
