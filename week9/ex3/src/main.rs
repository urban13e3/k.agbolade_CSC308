use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use chrono::Local;

fn main() {
    loop {
        println!("\n--- Note Taking App ---");
        println!("1. Add a note");
        println!("2. View all notes");
        println!("3. Exit");
        print!("Choose an option: ");
        io::stdout().flush().expect("flush failed");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("failed to read choice");

        match choice.trim() {
            "1" => add_note(),
            "2" => view_notes(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, try again."),
        }
    }
}

fn add_note() {
    print!("Enter your note: ");
    io::stdout().flush().expect("flush failed");

    let mut note = String::new();
    io::stdin().read_line(&mut note).expect("failed to read note");
    let note = note.trim();
    if note.is_empty() {
        println!("Empty note — not saved.");
        return;
    }

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let entry = format!("[{}] {}\n", timestamp, note);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("notes.txt")
        .expect("Could not open notes.txt for appending");

    file.write_all(entry.as_bytes()).expect("failed to write to notes.txt");
    println!("Note saved!");
}

fn view_notes() {
    let mut file = match OpenOptions::new().read(true).open("notes.txt") {
        Ok(f) => f,
        Err(_) => {
            println!("No notes found.");
            return;
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content).expect("failed to read notes.txt");

    println!("\n--- All Notes ---");
    if content.trim().is_empty() {
        println!("(No notes yet)");
    } else {
        println!("{}", content);
    }
}