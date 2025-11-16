use std::process::Command;

fn main() {
    let mut child = Command::new("echo")
        .arg("Hello from child process!")
        .spawn()
        .expect("Failed to spawn child process");

    println!("Child process spawned with PID: {}", child.id());

    
    let status = child.wait().expect("Failed to wait for child process");

    println!("Child process exited with status: {}", status);
}