use reqwest::Client;
use serde_json::json;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

async fn send_message(content: String) -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = "WebHookHere";

    let json_payload = json!({
        "content": content
    });

    let client = Client::new();

    let response = client.post(webhook_url)
        .json(&json_payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Message sent");
    } else {
        println!("Failed to send message: {:?}", response);
    }

    Ok(())
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    thread::spawn(move || {
        loop {
            print!("Enter message content (Type '/exit' to exit): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let content = input.trim();

            if content == "/exit" {
                running_clone.store(false, Ordering::Relaxed);
                break;
            }

            if let Err(err) = tokio::runtime::Runtime::new().unwrap().block_on(send_message(content.to_string())) {
                eprintln!("Error: {}", err);
            }
        }
    });

    while running.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(100));
    }
}
