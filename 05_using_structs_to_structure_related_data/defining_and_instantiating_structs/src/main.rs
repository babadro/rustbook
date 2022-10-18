struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // Creating Instances From Other Instances With Struct Update Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Using Tuple Structs without Named Fields to Create Different Types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Struct Without Any Fields
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // Ownership of Struct Data
    //struct User2 {
    //    active: bool,
    //    username: &str, // compile error: expected named lifetime parameter
    //    email: &str,
    //    sign_in_count: u64,
    //}
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Using the Field Init Shorthand
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
