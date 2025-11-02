use std::io;

fn main() {
    println!("Welcome to Smart Café!");
    println!("Please enter your total bill amount (₦):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let bill: f64 = input.trim().parse().expect("Please enter a valid number");

    let (discount_rate, discount_percent) = if bill > 10000.0 {
        (0.15, "15%")
    } else if bill > 5000.0 {
        (0.10, "10%")
    } else {
        (0.0, "0%")
    };

    let discount_amount = bill * discount_rate;
    let final_bill = bill - discount_amount;

    println!("\n--- Smart Café Receipt ---");
    println!("\n{}", format!("Original Bill: ₦{:.0}", bill));
    println!("{}", format!("Discount Applied: {}", discount_percent));
    println!("{}", format!("Final Bill: ₦{:.0}", final_bill));
    println!("Thank you for visiting Smart Café!");
}