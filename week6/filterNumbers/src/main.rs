fn main() {
    let numbers: Vec<u32> = (1..=20).collect();
    
    let even_numbers: Vec<u32> = numbers.iter()
        .filter(|&&n| n % 2 == 0)
        .copied()
        .collect();
    
    println!("Original numbers: {:?}", numbers);
    println!("Even numbers: {:?}", even_numbers);
}