#[derive(Debug)]
struct Rectangle(u32, u32);

impl Rectangle {
    fn show(&self) {
        println!("{:#?}", self)
    }

    fn square(i: u32) -> Rectangle {
        Rectangle(i, i)
    }
}

fn main() {
    let r = Rectangle(10, 20);
    r.show();
    let r = Rectangle::square(5);
    r.show();
}
