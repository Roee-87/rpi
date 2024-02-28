use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    // Spawn the second program
    let mut child = Command::new("../random/target/debug/random")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start program2");

    println!("making the request...");
    // Communicate with the second program
    if let Some(mut stdin) = child.stdin.take() {
        // Send a request for a random number
        writeln!(stdin, "generate_random_number").expect("Failed to write to stdin");
    }

    println!("request sent");

    // Wait for the second program to finish
    let output = child.wait_with_output().expect("Failed to wait for program2");

    // Print the result from the second program
    println!("Program 1 received: {:?}", String::from_utf8_lossy(&output.stdout));
}
