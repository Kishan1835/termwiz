use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};
use serde_json::json;
//phle request any tha ab string me aagya h 
pub async fn get_explanation(api_key: &str, command: &str) -> Result<String, String> {
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
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    // Check if the response was successful
    let status = res.status();
    if !status.is_success() {
        let error_text = res
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("API error ({}): {}", status, error_text));
    }

    // Parse the JSON 
    let json: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Check for API errors in the response
    if let Some(error) = json.get("error") {
        let error_msg = error
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("API returned an error");
        return Err(format!("API error: {}", error_msg));
    }

    // Extract the explanation or fallback
    Ok(json["candidates"]
        .get(0)
        .and_then(|c| c["content"]["parts"].get(0))
        .and_then(|p| p["text"].as_str())
        .unwrap_or("No explanation available.")
        .to_string())
}
