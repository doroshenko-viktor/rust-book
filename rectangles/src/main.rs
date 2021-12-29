#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn get_area(&self) -> i64 {
        (self.width * self.height) as i64
    }
}

fn main() {
    let rectangle = Rectangle {
        width: dbg!(2 + 4),
        height: 2,
    };
    dbg!("{}", &rectangle);
    println!("{:?}", rectangle);
    println!("verbose format: {:#?}", rectangle);
    let area = rectangle.get_area();
    println!("{}", area);
}
