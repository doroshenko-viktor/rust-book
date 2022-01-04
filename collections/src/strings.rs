fn creating_string() {
    let mut s = String::new();

    // when we have some init data:
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // create a String from a string literal
    let s = String::from("initial contents");
}

fn extending_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
}
