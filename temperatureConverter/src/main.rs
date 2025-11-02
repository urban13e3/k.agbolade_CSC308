use std::io; 

fn main() {
    println!("Enter temperature in Celsius:");

    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read input");

    let celsius: f64 = celsius.trim().parse().expect("Please type a number!");

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;

    println!("{}°C = {}°F", celsius, fahrenheit);
}