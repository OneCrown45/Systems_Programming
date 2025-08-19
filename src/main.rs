use std::fs;
use std::time::{Instant, Duration};
use chrono::{Utc, DateTime};
use reqwest::Client;
use tokio;
use std::sync::Arc;
use tokio::sync::Mutex;

struct WebsiteStatus {
    url: String,
    status: Result<u16, String>, // HTTP status code or error
    response_time: Duration,
    timestamp: DateTime<Utc>,
}

#[tokio::main]
async fn main() {
    // Load URLs from file
    let urls = fs::read_to_string("50Website.txt")
        .expect("Failed to read 50Website.txt")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    println!("Checking {} websites...\n", urls.len());

    // HTTP client with 5-second timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    // Shared vector to store results
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for url in urls {
        let client = client.clone();
        let results = results.clone();

        let handle = tokio::spawn(async move {
            // Normalize URL
            let url_clone = if url.starts_with("http") {
                url.clone()
            } else {
                format!("http://{}", url)
            };

            let start = Instant::now();
            let timestamp = Utc::now();

            let result = client.get(&url_clone).send().await;

            let website_status = match result {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    WebsiteStatus {
                        url: url_clone,
                        status: Ok(status),
                        response_time: start.elapsed(),
                        timestamp,
                    }
                }
                Err(e) => WebsiteStatus {
                    url: url_clone,
                    status: Err(e.to_string()),
                    response_time: start.elapsed(),
                    timestamp,
                },
            };

            // Store result
            results.lock().await.push(website_status);
        });

        handles.push(handle);
    }

    // Wait for all tasks to finish
    for h in handles {
        let _ = h.await;
    }

    // Print all results after completion
    let results = results.lock().await;
    for status in results.iter() {
        println!(
            "[{}] {} → {:?} ({} ms)",
            status.timestamp.format("%Y-%m-%d %H:%M:%S"),
            status.url,
            status.status,
            status.response_time.as_millis()
        );
    }
}
