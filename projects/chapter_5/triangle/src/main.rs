#[derive(Debug)] // you cannot derive Display to print in a normal formatter
struct Triangle {
    width: u32,
    height: u32,
}

impl Triangle {
    fn new(height: u32, width: u32) -> Self {
        Self { height, width }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    fn max(self, other: Rectangle) -> Self {
        Self {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn compute_area(&self) -> u32 {
        self.width * self.height / 2
    }

    fn can_fit(&self, other: &Triangle) -> bool {
        self.compute_area() >= other.compute_area()
    }
}

fn main() {
    let triangle = Triangle::new(dbg!(10 * 2), 15);
    let triangle2 = Triangle::new(30, 15);

    println!(
        "The area of the triangle is {} square pixels.",
        triangle.compute_area()
    );

    println!(
        "It's possible to fit triangle in triangle2: {}",
        triangle.can_fit(&triangle2)
    );

    println!(
        "It's possible to fit triangle2 in triangle: {}",
        triangle2.can_fit(&triangle)
    );

    println!("Triangle is {triangle:?}."); //:? makes it print with the debug formatter
    println!("Triangle is {triangle:#?}."); //:#? makes it print with the debug formatter but prettified
    dbg!(triangle); // even better way of doing it but remember that it takes ownership
}
