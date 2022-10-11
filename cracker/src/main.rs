use futures::future;
use reqwest::Client;
use std::error::Error;

const ADDR: &str = "http://localhost:9000";

struct TryPinResult {
    valid: bool,
    pin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pins = create_4_digit_pins()?;
    let try_pin_futures: Vec<_> = pins.into_iter().map(move |pin| try_pin(pin)).collect();
    let results: Vec<_> = future::join_all(try_pin_futures)
        .await
        .into_iter()
        .filter_map(|result| result.ok())
        .collect();
    for result in results {
        if result.valid {
            println!("Successfully cracked pin: {}", result.pin);
            return Ok(());
        }
    }
    println!("Failed to crack pin");
    Ok(())
}

fn create_4_digit_pins() -> Result<Vec<String>, Box<dyn Error>> {
    let mut pins: Vec<String> = vec![];
    for i in 0..1500 {
        let pin = format!("{:0>4}", i.to_string());
        pins.push(pin.to_string());
    }
    Ok(pins)
}

async fn try_pin(pin: String) -> Result<TryPinResult, Box<dyn Error>> {
    let client = Client::new();
    println!("Request initiated for pin: {}", pin);
    let resp = client
        .post(format!("{}/try_pin?pin={}", ADDR, pin))
        .send()
        .await?;
    println!("Requested for pin: {}", pin);
    if resp.status() != 200 {
        return Err("Not 200 code".into());
    }

    let body = resp.text().await?;
    if body.contains("Incorrect") {
        println!("Pin {} is incorrect", pin);
    } else {
        println!("Pin {} is correct!!!", pin);
        return Ok(TryPinResult { valid: true, pin });
    }
    Ok(TryPinResult { valid: false, pin })
}
