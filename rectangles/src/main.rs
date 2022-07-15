#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        height: 40,
        width: 50,
    };

    println!("rectangle is {:#?}", rectangle);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
