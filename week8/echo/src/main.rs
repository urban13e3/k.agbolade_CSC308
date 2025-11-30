use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("Running command: echo 'Rust Process Management'");
    
    // Run the echo command and capture its output
    let output = Command::new("echo")
        .arg("Rust Process Management")
        .output()
        .expect("Failed to execute echo command");
    
    // Check if the command was successful
    if output.status.success() {
        println!("Command executed successfully!");
        
        // Convert the stdout bytes to a string
        let stdout_text = String::from_utf8_lossy(&output.stdout);
        println!("Captured output: {}", stdout_text.trim());
        
        // Write the output to a file
        let mut file = File::create("output.txt")
            .expect("Failed to create output.txt");
        
        file.write_all(&output.stdout)
            .expect("Failed to write to output.txt");
        
        println!("Output written to output.txt");
        println!("\nVerify with: cat output.txt");
    } else {
        eprintln!("Command failed with exit code: {:?}", output.status.code());
        
        // Print any error output
        let stderr_text = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error output: {}", stderr_text);
    }
}

