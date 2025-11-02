use std::io;

fn lastWord(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut last_space = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            last_space = i + 1;
        }
    }

    &s[last_space..]
}
fn main() {
    println!("Enter your sentence: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let result = lastWord(&input);
    println!("Last word: {}", result)
}
