struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

fn main() {
    use std::io;

    println!("Enter the radius of the circle:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let radius: f64 = input.trim().parse().expect("Please enter a valid number");

    let circle = Circle { radius };
    println!("Area: {}", circle.area());
    println!("Circumference: {}", circle.circumference());
}
