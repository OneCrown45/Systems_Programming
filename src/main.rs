use std::{thread, time::{Duration, Instant}};
use chrono::{DateTime, Utc};
use std::sync::{mpsc, Arc, Mutex};
use ureq::Agent;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}

fn check_status(url: String, timeout: Duration) -> WebsiteStatus {
    let start = Instant::now();
    let agent: Agent = Agent::new();             

    let resp = match agent.get(&url)
        .timeout(timeout)
        .call() {
            Ok(status) => Ok(status.status()),
            Err(e) => Err(e.to_string()),
        };

    WebsiteStatus {
        url,
        status: resp,
        response_time: start.elapsed(),
        timestamp: Utc::now(),
    }
}

fn main() {
    // Load URLs from file
    let file = File::open("50Websites.txt").expect("Couldn't open file");
    let buffer = BufReader::new(file);

    let mut urls: Vec<String> = Vec::new();
    for line in buffer.lines() {
        if let Ok(url) = line {
            urls.push(url);
        }
    }

    // Optimized settings
    let timeout = Duration::from_secs(2);   // 2-second timeout
    let num_workers = urls.len();           // one worker per URL
    let (tx, rx) = mpsc::channel::<String>();
    let (result_tx, result_rx) = mpsc::channel::<WebsiteStatus>();
    let rx = Arc::new(Mutex::new(rx));

    // Spawn workers
    for _ in 0..num_workers {
        let rx = Arc::clone(&rx);
        let result_tx = result_tx.clone();
        let timeout = timeout.clone();
        
        thread::spawn(move || {
            while let Ok(url) = rx.lock().unwrap().recv() {
                let status = check_status(url, timeout);
                result_tx.send(status).unwrap();
            }
        });
    }

    // Send all URLs then close channel
    for url in urls {
        tx.send(url).unwrap();
    }
    drop(tx);

    // Collect results
    for result in result_rx {
        match result.status {
            Ok(code) => println!(
                "[{}] ✅ {} responded with {} in {:?}",
                result.timestamp, result.url, code, result.response_time
            ),
            Err(err) => println!(
                "[{}] ❌ {} failed: {}",
                result.timestamp, result.url, err
            ),
        }
    }
}
