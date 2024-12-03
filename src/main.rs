use reqwest::Error;
use std::env;
use std::thread;
use std::time::Duration;

async fn send_airtable_call() -> Result<(), Error> {
    let local_airtable_base_id = env::var("LOCAL_AIRTABLE_BASE_ID")
        .expect("LOCAL_AIRTABLE_BASE_ID environment variable is not set");
    let airtable_token =
        env::var("AIRTABLE_TOKEN").expect("AIRTABLE_TOKEN environment variable is not set");

    let client = reqwest::Client::new();

    let url = format!(
        "https://api.airtable.com/v0/meta/bases/{}/tables",
        local_airtable_base_id
    );

    let resp = client.get(&url).bearer_auth(airtable_token).send().await?;

    println!("{:#?}", resp);

    Ok(())
}

#[tokio::main]
async fn main() {
    loop {
        if let Err(e) = send_airtable_call().await {
            eprintln!("Error making Airtable API call: {:?}", e);
        }

        thread::sleep(Duration::from_secs(20 * 60));
    }
}
