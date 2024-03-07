use reqwest;
use reqwest::Client;
use serde_json;
use serde::{Serialize, Deserialize};
use std::env;
use std::fs::read_to_string;
use tokio;

#[derive(Serialize, Deserialize)]
pub struct RespObj {
    #[serde(rename = "candidates")]
    candidates: Vec<Candidate>,

    #[serde(rename = "promptFeedback")]
    prompt_feedback: PromptFeedback,
}

#[derive(Serialize, Deserialize)]
pub struct Candidate {
    #[serde(rename = "content")]
    content: Content,

    #[serde(rename = "finishReason")]
    finish_reason: String,

    #[serde(rename = "index")]
    index: i64,

    #[serde(rename = "safetyRatings")]
    safety_ratings: Vec<SafetyRating>,
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "parts")]
    parts: Vec<Part>,

    #[serde(rename = "role")]
    role: String,
}

#[derive(Serialize, Deserialize)]
pub struct Part {
    #[serde(rename = "text")]
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct SafetyRating {
    #[serde(rename = "category")]
    category: String,

    #[serde(rename = "probability")]
    probability: String,
}

#[derive(Serialize, Deserialize)]
pub struct PromptFeedback {
    #[serde(rename = "safetyRatings")]
    safety_ratings: Vec<SafetyRating>,
}


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

fn extract_text_field(data: &str) -> Result<Vec<String>, serde_json::Error> {
    let parsed_data: RespObj = serde_json::from_str(data)?;
    let mut text_fields = vec![];
    for content in parsed_data.candidates.iter() {
      for part in content.content.parts.iter() {
        text_fields.push(part.text.clone());
      }
    }
    Ok(text_fields)
  }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = r#"{
       "contents": [
           {
               "parts": [
                   {
                       "text": "What is the Rust programming language? Respond in plain text."
                   }
               ]
           }
       ]
    }"#;

    let response_body = make_request(data).await?;
    let text_fields = extract_text_field(&response_body)?;
    println!("{}", text_fields[0]);

    Ok(())
}
