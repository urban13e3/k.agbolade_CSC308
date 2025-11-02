use std::io;

struct Student {
    name: String,
    score: f64,
}

impl Student {
    fn has_passed(&self) -> bool {
        self.score >= 50.0
    }

    fn display_result(&self) {
        if self.has_passed() {
            println!("{} passed with a score of {:.2}.", self.name, self.score);
        } else {
            println!("{} failed with a score of {:.2}.", self.name, self.score);
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter the student's name:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let name = input.trim().to_string();

    input.clear();
    println!("Enter the student's score:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let score: f64 = input.trim().parse().expect("Please enter a valid number for the score");

    let student = Student { name, score };

    
    student.display_result();
}