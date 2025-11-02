use std::io;

fn main() {
    println!("Enter a sentence:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let words: Vec<&str> = input
        .trim()
        .split_whitespace()
        .collect();

    if words.is_empty() {
        println!("No words found in the sentence.");
        return;
    }

    let mut longest = words[0];
    let mut shortest = words[0];

    for &word in &words {
        if word.len() > longest.len() {
            longest = word;
        }
        if word.len() < shortest.len() {
            shortest = word;
        }
    }

    println!("Longest word: {}", longest);
    println!("Shortest word: {}", shortest);
}