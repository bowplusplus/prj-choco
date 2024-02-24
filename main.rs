struct User {
    name: String,
    email: String,
}

impl User {
    fn new(name: String, email: String) -> User {
        User { name, email }
    }

    fn display(&self) {
        println!("Name: {}", "Email: {}", self.name, self.email);
    }
}

fn main() {
    let user = User::new(
        "test_user".to_string(),
        "test@mail.com".to_string(),
    );
    user.display();
}