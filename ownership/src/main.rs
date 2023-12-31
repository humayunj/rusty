struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let u = build_user(String::from("h@gmail.com"), String::from("hj"));

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
