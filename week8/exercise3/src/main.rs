use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting long-running process: ping google.com");
    
    // Spawn the ping process
    let mut child = Command::new("ping")
        .arg("google.com")
        .spawn()
        .expect("Failed to start ping process");
    
    let child_pid = child.id();
    println!("Ping process started with PID: {}", child_pid);
    println!("\n=== Quick! Run this in another terminal ===");
    println!("top -p {}", child_pid);
    println!("or: ps -p {}", child_pid);
    println!("\nYou should see the ping process running!");
    
    // Wait for 5 seconds while the process runs
    println!("\nWaiting 5 seconds...");
    for i in 1..=5 {
        println!("  {}...", i);
        thread::sleep(Duration::from_secs(1));
    }
    
    // Kill the child process
    println!("\nKilling the ping process...");
    match child.kill() {
        Ok(_) => println!("Process killed successfully"),
        Err(e) => eprintln!("Failed to kill process: {}", e),
    }
    
    // Wait for the process to fully terminate and get its exit status
    match child.wait() {
        Ok(status) => {
            println!("\n=== Exit Status ===");
            println!("Process exited with: {}", status);
            
            // Check if it was terminated by a signal
            if !status.success() {
                println!("Process was terminated (not a clean exit)");
                if let Some(code) = status.code() {
                    println!("Exit code: {}", code);
                } else {
                    println!("Process was killed by signal (no exit code)");
                }
            }
        }
        Err(e) => eprintln!("Failed to wait for process: {}", e),
    }
    
    println!("\n=== Verification ===");
    println!("In the terminal, run: echo $?");
    println!("This shows the exit code of the last command (this Rust program)");
    println!("Note: The ping process was killed, so it won't have a normal exit code");
}

