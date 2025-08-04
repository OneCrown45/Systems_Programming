use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct LinuxAgent {
    path: String,
}

impl LinuxAgent {
    fn new() -> LinuxAgent {
        let path = "command_history.txt".to_string();
        File::create(&path).expect("Failed to create history file");
        LinuxAgent { path }
    }

    fn executing_os_commands_linux(&self, command_full: &str) {
        let mut parts = command_full.trim().split_whitespace();
        let command = match parts.next() {
            Some(c) => c,

            
            None => {
                println!("No command entered.");
                return;
            }
        };
        let args: Vec<&str> = parts.collect();

        let output = Command::new(command)
            .args(&args)
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                let log = format!(
                    "\n[{}] Command: {}
Output:
{}
Errors:
{}\n",
                    timestamp, command_full, stdout, stderr
                );

                println!("{}", log);
                self.save_results(log);
            }
            Err(e) => {
                let log = format!("\nCommand failed to execute: {}\n", e);
                println!("{}", log);
                self.save_results(log);
            }
        }
    }

    fn accept_linux_command_from_user() -> String {
        println!("Enter a Linux command (or type 'exit' to quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }

    fn save_results(&self, content: String) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
            .expect("Unable to open file");

        writeln!(file, "{}", content).expect("Unable to write to file");
    }

    fn show_results(&self) {
        let file = File::open(&self.path).expect("Unable to open history file");
        let reader = BufReader::new(file);

        println!("\nCommand History:\n-------------------");
        for line in reader.lines() {
            if let Ok(l) = line {
                println!("{}", l);
            }
        }
    }
}

fn main() {
    let agent = LinuxAgent::new();

    loop {
        let command = LinuxAgent::accept_linux_command_from_user();
        if command.eq_ignore_ascii_case("exit") {
            break;
        }
        agent.executing_os_commands_linux(&command);
    }

    agent.show_results();
}
