// Define a trait for shapes
trait Shape {
    fn area(&self) -> f64;
}

// Implement the trait for a Rectangle
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Implement the trait for a Circle
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let rectangle = Rectangle { width: 5.0, height: 3.0 };
    let circle = Circle { radius: 2.0 };

    println!("Area of Rectangle: {}", rectangle.area());
    println!("Area of Circle: {}", circle.area());
}
