fn main() {
    let user1 = User {
        username: String::from("Wenhao Tong"),
        email: String::from("854772137@qq.com"),
        sign_in_count: 0,
        active: true,
    };

    let mut user2 = User {
        username: String::from("Wenhao Tong"),
        email: String::from("854772137@qq.com"),
        sign_in_count: 0,
        active: true,
    };
    user2.active = false;

    let user3 = User {
        email: String::from("hello@qq.com"),
        username: String::from("Sakamoto"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: false,
        sign_in_count: 0,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
