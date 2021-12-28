pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn struct_instantiating() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);
}

// it is not necessary to repeate field_name:value pattern if field name and source value have same name
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn create_user_copy_with_update_syntax() -> User {
    let user1 = User {
        email: "email".to_string(),
        username: "username".to_string(),
        active: true,
        sign_in_count: 1,
    };

    // struct update syntax is like assignment with = because it moves the data
    // In this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2
    let user2 = User {
        email: "new@email.com".to_string(),
        ..user1
    };
    user2
}

// TUPLE STRUCTS

// Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each field as in a regular struct would be verbose or redundant.

fn create_tuple_structs_example() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // black and origin values are different types, because they’re instances of different tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself
fn create_unit_like_struct() {
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

