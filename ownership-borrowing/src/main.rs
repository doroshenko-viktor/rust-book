mod borrowing_example;

fn main() {}

pub fn borrowing_example() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

pub fn calculate_length(s: &String) -> usize {
    s.len()
}

#[test]
fn test_borrowing_example() {
    borrowing_example();
}

pub fn mutable_borrowing_example() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[test]
fn test_mutable_borrowing_example() {
    mutable_borrowing_example();
}

// multiple mutable references are not allowed, but it is possible to have multiple mutable
// references with correct lifetimes in scope
pub fn multiple_mutable_references() {
    let mut s = String::from("hello;");

    {
        let r1 = &mut s;
        r1.push_str(" from r1;");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    r2.push_str(" from r2;");
    println!("{}", s);
}

#[test]
fn test_multiple_mutable_references() {
    multiple_mutable_references()
}

// We also cannot have a mutable reference while we have an immutable one. Users of an immutable reference don’t expect the values to suddenly change out from under them! However, multiple immutable references are okay because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
// But it is possible to have multiple immutable references with mutable reference,
// when they are live in different scopes. Scope counts from the place, where reference
// is declared and until it's last usage
fn multiple_mutable_and_immutable_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    // println!("{} and {}", r1, r2); // this is not possible
}

#[test]
fn test_multiple_mutable_and_immutable_references() {
    multiple_mutable_and_immutable_references()
}
