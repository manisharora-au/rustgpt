#[derive(Debug)]
#[derive(PartialEq)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

impl User {
    fn new(username: String, email: String) -> User {
        User {
            username: username,
            email: email,
            sign_in_count: 0,
            active: true,
        }
    }

    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, email: String) {
        self.email = email;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        // Create a new user
        let mut user_1 = User::new(
            String::from("John Doe"),
            String::from("johndoe@example.com"),
        );
        user_1.change_email(String::from("manisharora.au@gmail.com"));
        dbg!(user_1);
    }
}