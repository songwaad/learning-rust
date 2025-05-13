fn main() {
    let user1:User = User {
        username: "username".to_string(),
        email: "example@email.com".to_string(),
        sign_in_count: 1,
        active: true,
    };

    print!("Username : {}", user1.username);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}