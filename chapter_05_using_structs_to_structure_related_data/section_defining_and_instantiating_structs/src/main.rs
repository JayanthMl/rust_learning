struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple type structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// Unit-Like structs
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("userhahaha"),
        email: String::from("user@email.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("newuseremail@example.com");

    // Creating a new instance using 
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Creating a new instance using the update syntax, the ..user2 tells that the rest of the fields are the same as user2
    let user3 = User {
        email: String::from("anotherfrom@example.com"),
        ..user2
    };
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// The same fucntion can be re-written using shorthands like below
fn build_user_shorthand(email:String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}