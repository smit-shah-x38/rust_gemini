use reqwest;
use reqwest::Client;
use serde_json;
use std::env;
use std::fs::read_to_string;
use tokio;

async fn make_request(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let clientbuild = Client::builder();
    let client = clientbuild.build()?;
    let api_key = env::var("GOOGLE_API_KEY")
        .or_else(|_| read_to_string("E:\\Work\\workbackups\\localdata\\googleapi.txt"))
        .expect("Failed to read API key");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let json: serde_json::Value = serde_json::from_str(data)?;
    let stringer = json.to_string();
    let request = client.request(reqwest::Method::POST, "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=".to_owned() + &api_key)
       .headers(headers)
       .body(stringer);

    let response = request.send().await?;
    let body = response.text().await?;

    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = r#"{
       "contents": [
           {
               "parts": [
                   {
                       "text": "Write a story about a magic backpack."
                   }
               ]
           }
       ]
   }"#;

    let response_body = make_request(data).await?;
    println!("Actual Response: {}", response_body);

    Ok(())
}
