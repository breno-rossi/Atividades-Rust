use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, PartialEq)]
struct User {
    name: String,
    username: String,
    password: String,
    privilege: String,
}

#[derive(Debug, PartialEq)]
enum LoginResult {
    Success { name: String, privilege: String },
    UserNotFound,
    IncorrectPassword,
}

// Function to read users from a JSON file
fn read_users_from_file(file_path: &str) -> Result<Vec<User>, String> {
    //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // To complete
    //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
}

fn login(username: &str, password: &str, users: &[User]) -> LoginResult {
    //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // To complete
    //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
}

fn main() {
    // Load users from JSON file
    let users = match read_users_from_file("users.json") {
        //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // To complete
        //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    };

    // Simulate login
    let username = "alicej";
    let password = "alice123";

    match login(username, password, &users) {
        //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // To complete
        //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_users() -> Vec<User> {
        vec![
            User {
                name: String::from("Alice Johnson"),
                username: String::from("alicej"),
                password: String::from("alice123"),
                privilege: String::from("admin"),
            },
            User {
                name: String::from("Bob Smith"),
                username: String::from("bobsmith"),
                password: String::from("bobpassword"),
                privilege: String::from("user"),
            },
        ]
    }

    #[test]
    fn test_login_success() {
        let users = get_test_users();
        let result = login("alicej", "alice123", &users);
        assert_eq!(
            result,
            LoginResult::Success {
                name: String::from("Alice Johnson"),
                privilege: String::from("admin")
            }
        );
    }

    #[test]
    fn test_login_user_not_found() {
        let users = get_test_users();
        let result = login("unknownuser", "password", &users);
        assert_eq!(result, LoginResult::UserNotFound);
    }

    #[test]
    fn test_login_incorrect_password() {
        let users = get_test_users();
        let result = login("alicej", "wrongpassword", &users);
        assert_eq!(result, LoginResult::IncorrectPassword);
    }

    #[test]
    fn test_login_user_bob() {
        let users = get_test_users();
        let result = login("bobsmith", "bobpassword", &users);
        assert_eq!(
            result,
            LoginResult::Success {
                name: String::from("Bob Smith"),
                privilege: String::from("user")
            }
        );
    }
}
