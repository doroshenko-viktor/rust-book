pub fn simple_slices_example() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2]; //this two are equal

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..]; // this two also same

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..]; // and these two are equal
}

// this function returns string slice of first word in the given string
// &str is a type signature, conforming to string slice
pub fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn test_first_word() {
    let s = String::from("first second third");
    let first_word = get_first_word(&s);
    // drop(s); // cannot drop s, because it is borrowed by slice in first_word
    print!("first word: {}", first_word);
}

#[test]
fn simple_array_slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
