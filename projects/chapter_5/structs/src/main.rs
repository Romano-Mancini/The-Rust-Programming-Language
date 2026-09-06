struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i32,
}

fn new_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = new_user(
        String::from("romano_mancini"),
        String::from("sampleemail@test.com"),
    );

    let user2 = User {
        email: String::from("test2@test.com"),
        ..user1
    };

    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
}
