#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let rectangle = Rectangle {
        width: dbg!(2 + 4),
        height: 2,
    };
    dbg!("{}", &rectangle);
    println!("{:?}", rectangle);
    println!("verbose format: {:#?}", rectangle);
    let area = get_area(&rectangle);
    println!("{}", area);
}

fn get_area(rectangle: &Rectangle) -> i64 {
    (rectangle.width * rectangle.height) as i64
}
