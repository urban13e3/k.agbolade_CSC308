use std::io;

fn main() {
    println!("Welcome to Smart Energy Company (SEC)!");
    println!("Enter your electricity usage in kWh:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let usage: f64 = input.trim().parse().expect("Please enter a valid number");

    let rate = if usage > 200.0 {
        30.0
    } else if usage > 100.0 {
        25.0
    } else {
        20.0
    };

    let total_bill = usage * rate;

    println!("\n--- Smart Energy Bill ---");
    println!("Energy Used: {:.0} kWh", usage);
    println!("Rate per kWh: ₦{:.0}", rate);
    println!("Total Bill: ₦{:.2}", total_bill);
}