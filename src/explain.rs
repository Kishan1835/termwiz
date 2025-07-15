use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};
use serde_json::json;

pub async fn get_explanation(api_key: &str, command: &str) -> Result<String, reqwest::Error> {
    // Prepare the request body
    let body = json!({
        "contents": [{
            "parts": [{
    "text": format!("In 2 short sentences, explain simply what this terminal command does: {}", command)
            }]
        }]
    });

    // Build headers with API key and content-type
    let mut headers = HeaderMap::new();
    headers.insert("X-goog-api-key", HeaderValue::from_str(api_key).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Send the request to the Gemini API
    let res = Client::new()
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent")
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    // Parse the JSON response
    let json: serde_json::Value = res.json().await?;

    // Extract the explanation or fallback
    Ok(json["candidates"]
        .get(0)
        .and_then(|c| c["content"]["parts"].get(0))
        .and_then(|p| p["text"].as_str())
        .unwrap_or("No explanation available.")
        .to_string())
}
