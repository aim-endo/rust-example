
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);
struct Point(i32, i32);

fn main() {
    let mut u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    u1.email = String::from("another@example.com");
    println!("Hello, {}", u1.username);

    let u2 = build_user(String::from("example@example.com"), "myname".to_string());
    println!("Hello, {} ... {}", u2.username, u2.email);

    let u3 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        ..u1
    };
    println!("Hello, {} ... {}", u3.username, u3.email);

    // tuple structure
    let black = Color(0, 0, 0);
    let origin = Point(0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: false,
        sign_in_count: 0,
    }
}
