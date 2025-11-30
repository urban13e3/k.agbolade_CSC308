use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Parent Process PID: {}", std::process::id());
    println!("Starting three child processes...\n");

    // Child 1: sleep 5
    let mut child1 = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to start sleep process");
    println!("Child 1 (sleep 5) started with PID: {}", child1.id());

    // Child 2: ls -la
    let mut child2 = Command::new("ls")
        .arg("-la")
        .spawn()
        .expect("Failed to start ls process");
    println!("Child 2 (ls -la) started with PID: {}", child2.id());

    // Child 3: echo "Hello from child"
    let mut child3 = Command::new("echo")
        .arg("Hello from child")
        .spawn()
        .expect("Failed to start echo process");
    println!("Child 3 (echo) started with PID: {}", child3.id());

    println!("\n=== Quick! Run these commands in another terminal ===");
    println!("ps -ef | grep {}", std::process::id());
    println!("pstree -p {}", std::process::id());
    println!("top -p {},{},({})", child1.id(), child2.id(), child3.id());
    
    // Give time to observe the processes
    println!("\nWaiting 2 seconds for you to observe processes...");
    thread::sleep(Duration::from_secs(2));

    // Wait for all children to complete
    let status1 = child1.wait().expect("Failed to wait for child1");
    let status2 = child2.wait().expect("Failed to wait for child2");
    let status3 = child3.wait().expect("Failed to wait for child3");

    println!("\n=== Process Status ===");
    println!("Child 1 (sleep) exited with: {}", status1);
    println!("Child 2 (ls) exited with: {}", status2);
    println!("Child 3 (echo) exited with: {}", status3);
}

