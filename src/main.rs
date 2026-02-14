struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let user1 = build_user(
        String::from("Sheryar"),
        String::from("Sheryar@gmail.com"), 
    );

    let user2 = User {
        email: String::from("Tahir@gmail.com"),
        ..user1
    };

    println!("Email of user is: {}", user2.email);
}

fn build_user(username: String, email: String) -> User {
    User { 
        active: true,
        username,
        email,
        sign_in_count: 0 
    }
}