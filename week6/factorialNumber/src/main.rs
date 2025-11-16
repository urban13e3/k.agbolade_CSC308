fn factorial(num: u64) -> u64 {
    let mut result = 1;
    (1..=num).for_each(|i| result *= i);
    result
}

fn main() {
    use std::io;

    println!("Enter your number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: u64 = input.trim().parse().expect("Please enter a valid number!");

    let fact = factorial(num);
    println!("{}! = {}", num, fact);
}