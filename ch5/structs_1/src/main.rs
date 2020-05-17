
// Create a struct for a user type, use String not slices or string literals
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// a function that takes in self named params and assignes them to the struct
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // create a mutable user
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // update a prop for the variable
    user1.email = String::from("anotheremail@example.com");

    // call the function that returns a User
    let _user = build_user(String::from("poo"), String::from("test_user_name"));

    // flatten user1 to reuse unchanged properties
    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

}
