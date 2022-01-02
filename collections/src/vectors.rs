fn creating_vector() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
}

fn adding_vector_elements() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn accessing_vec_elements() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; // this will panic
    let does_not_exist = v.get(100); // this returns Result enum
}

fn cant_have_mut_and_immut_ref() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); // cannot borrow here
    // This error is due to the way vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.
    println!("The first element is: {}", first);
}

fn iteration_with_for() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    // with mutability
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

// when we need to store elements of a different type in a vector, we can define and use an enum
fn store_mult_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
