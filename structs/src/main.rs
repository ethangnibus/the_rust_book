struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    return User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    };
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email:String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let mut user2 = build_user(
        String::from("ethan@example.com"),
        String::from("ettthhaann")
    );

    user2.email = String::from("hello@example.com");


    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
